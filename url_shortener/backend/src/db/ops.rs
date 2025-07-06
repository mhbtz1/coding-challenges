
use tokio_postgres::{Config, NoTls, Client, Connection, Socket};
use anyhow::Result;

pub async fn spawn_client(host: &str, port: &str, username: &str, password: &str, db_name: &str) -> Result<(Client, Connection<Socket, NoTls::Stream>)> {
    let mut cfg = Config::new();
    cfg.user(username)
    .password(password)
    .host(host)
    .port(port)
    .dbname(db_name)
    .ssl_mode(tokio_postgres::config::SslMode::Disable);

    cfg.connect(NoTls).await? // returns tuple containing client and connection.
}
