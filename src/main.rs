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

use assets::assets;
use axum::Extension;
use axum::Router;
use axum::routing::delete;
use axum::routing::get;
use hyper::Server;

mod config;
mod mpd;
mod route;
mod time;
mod tracing;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), String> {
    let handle = tracing::init();

    let config = config::read().await?;

    handle.set_level(config.logging.level)?;

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

    let api = Router::new()
        .route("/ws", get(route::ws::websocket))
        .route("/database", get(route::db::database))
        .route("/database/cover", get(route::db::cover))
        .route("/database/count", get(route::db::count))
        .route("/playlists", get(route::playlists::playlists))
        .route("/playlists/:name", get(route::playlists::playlist).delete(route::playlists::delete))
        .route("/playlists/:name/songs", delete(route::playlists::delete_songs))
        .layer(Extension(handle))
        .layer(Extension(sub_handle));

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
