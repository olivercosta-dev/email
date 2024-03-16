use email::configuration::get_configuration;
use email::startup::run;
use email::telemetry::*;
use sqlx::postgres:: PgPoolOptions;
use std::net::TcpListener;



#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber(
        "email".into(), "info".into(), std::io::stdout);
    
    init_subscriber(subscriber);
    
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPoolOptions::new()
        // `connect_lazy_with` instead of `connect_lazy`
        .connect_lazy_with(configuration.database.with_db());
    
    let address = format!("{}:{}", configuration.application.host, configuration.application.port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await?;
    Ok(())
}
