use tracing::Level;
use tracing_subscriber::filter::Targets;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::Layered;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;
use tracing_subscriber::reload;
use tracing_subscriber::util::SubscriberInitExt;

const TARGET: &str = "mpdweb";

pub struct Handle {
    reload: reload::Handle<Targets, Layered<fmt::Layer<Registry>, Registry>>,
}

impl Handle {
    pub fn set_level(&self, level: Level) -> Result<(), String> {
        self.reload
            .modify(|filter| {
                *filter = Targets::new().with_target(TARGET, level);
            })
            .map_err(|e| e.to_string())
    }
}

pub fn init() -> Handle {
    let filter = Targets::new()
        .with_target(TARGET, Level::INFO);

    let (filter, reload) = reload::Layer::new(filter);

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .init();

    Handle { reload }
}
