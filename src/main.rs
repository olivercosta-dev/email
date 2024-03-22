use email::configuration::get_configuration;
use email::email_client::EmailClient;
use email::startup::{ run, Application};
use email::{email_client, telemetry::*};
use sqlx::postgres:: PgPoolOptions;
use std::net::TcpListener;
use email::configuration::Settings;


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber(
        "email".into(), "info".into(), std::io::stdout);
    
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())

}
