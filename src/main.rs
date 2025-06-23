use tracing_subscriber::{self, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use ddai::exec;

fn main() {
    // Initialize tracing subscriber for logging
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("ddai=info,ddai::core=error"));

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)
                .with_level(true),
        )
        .with(env_filter)
        .init();

    exec();
}
