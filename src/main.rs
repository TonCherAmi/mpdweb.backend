#![warn(clippy::print_stdout)]
#![warn(clippy::if_then_some_else_none)]
#![warn(clippy::match_wildcard_for_single_variants)]
#![deny(clippy::empty_structs_with_brackets)]
#![deny(clippy::mod_module_files)]
#![deny(clippy::needless_collect)]
#![deny(clippy::needless_continue)]
#![deny(clippy::redundant_else)]
#![deny(clippy::redundant_closure_for_method_calls)]
#![deny(clippy::semicolon_if_nothing_returned)]
#![deny(clippy::wildcard_imports)]

use std::collections::HashMap;
use std::net::SocketAddr;
use std::process;

use assets::assets;
use axum::Extension;
use axum::Router;
use axum::routing::delete;
use axum::routing::get;
use hyper::Server;

mod args;
mod config;
mod mpd;
mod route;
mod time;
mod tracing;
mod persist;
mod history;
mod convert;
mod labels;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), String> {
    let args = match args::read() {
        Ok(args) => args,
        Err(usage) => {
            eprintln!("{}", usage);

            process::exit(1);
        }
    };

    let config = config::read(&args).await?;

    let persistence_handle = persist::init(config.database.path).await?;

    let handle = tracing::init();

    handle.set_level(config.logging.level)?;

    let labels_handle = labels::Handle::new(persistence_handle.clone());
    let history_handle = history::Handle::new(persistence_handle.clone());

    let handle = mpd::Handle::new(move || {
        let host = config.mpd.host.clone();
        let port = config.mpd.port;

        let password = config.mpd.password.clone();

        async move {
            let addr = format!("{host}:{port}");

            let mut client = mpd::connect(&addr).await?;

            if let Some(password) = password {
                client.password(password).await?;
            }

            Ok(client)
        }
    });

    let sub_handle = mpd::SubscriptionHandle::new(handle.clone());

    history::keeper::run(
        handle.clone(),
        sub_handle.clone(),
        persistence_handle.clone(),
    );

    let api = Router::new()
        .route("/ws", get(route::ws::websocket))
        .route("/database", get(route::db::database))
        .route("/database/cover", get(route::db::cover))
        .route("/database/count", get(route::db::count))
        .route("/database/recents", get(route::db::recents))
        .route("/playlists", get(route::playlists::playlists))
        .route("/playlists/:name", get(route::playlists::playlist).delete(route::playlists::delete))
        .route("/playlists/:name/songs", delete(route::playlists::delete_songs))
        .route("/history", get(route::history::history))
        .route("/labels", get(route::labels::labels).post(route::labels::create))
        .route("/labels/:id", delete(route::labels::delete))
        .layer(Extension(handle))
        .layer(Extension(sub_handle))
        .layer(Extension(labels_handle))
        .layer(Extension(history_handle));

    let app = Router::new()
        .nest("/api", api);

    let app = 'block: {
        let Some(files): Option<HashMap<&str, &[u8]>> = assets!() else {
            break 'block app;
        };

        app.route("/", get(route::assets::assets))
            .route("/*assets", get(route::assets::assets))
            .layer(Extension(files))
    };

    let addr = format!("{}:{}", config.server.bind, config.server.port)
        .parse::<SocketAddr>()
        .map_err(|e| e.to_string())?;

    Server::try_bind(&addr)
        .map_err(|e| e.to_string())?
        .serve(app.into_make_service())
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
