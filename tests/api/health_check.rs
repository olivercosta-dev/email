use std::net::TcpListener;
use reqwest;
use email::{configuration::{get_configuration, DatabaseSettings}, email_client::EmailClient, startup::run, telemetry::{get_subscriber, init_subscriber}};
use sqlx::{types::Uuid, Connection, PgConnection, PgPool, Executor};
use once_cell::sync::Lazy;


pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}
impl TestApp {
    pub async fn post_subscriptions(&self, body: String) -> reqwest::Response {
        reqwest::Client::new()
            .post(&format!("{}/subscriptions", &self.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

#[tokio::test]
async fn health_check_works() {
    
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}