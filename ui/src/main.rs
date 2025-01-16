use log::error;
use log::LevelFilter;
use tracing_subscriber::prelude::*;
use util::AppLogger;

static LOGGER: AppLogger = AppLogger {};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    log::set_logger(&LOGGER).unwrap();
    match ui::get_args() {
        Ok(config) => {
            // TODO ログレベル置き換え
            log::set_max_level(LevelFilter::Trace);

            if let Err(e) = ui::run(&config).await {
                error!("{}", e);
                std::process::exit(1);
            }
        }
        Err(e) => {
            error!("{}", e);
            std::process::exit(1);
        }
    }
}
