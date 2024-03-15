use email::configuration::get_configuration;
use email::startup::run;
use email::telemetry::*;
use sqlx::postgres::PgPool;
use std::net::TcpListener;
use secrecy::ExposeSecret;



#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber(
        "email".into(), "info".into(), std::io::stdout);
    
    init_subscriber(subscriber);
    
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect_lazy(
        &configuration.database.connection_string().expose_secret()
    )
    .expect("Failed to create Postgres connection pool.");
    
    let address = format!("127.0.0.1:{}", configuration.application.port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await?;
    Ok(())
}
