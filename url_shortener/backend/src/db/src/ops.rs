
use std::fs;
use tokio_postgres::{Config, NoTls, Client, Connection, Socket};
use anyhow::{Result, Error};
use std::Option;
use std::vec::Vec;

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

pub async fn create_table(table_name: &str) {
    let rows = client.query(fs::read_to_string("queries/init_table.sql")?, &[table_name]).await?;
}

pub async fn add_url_mapping(table_name: &str, url: &str, detinified_url: &str) -> Result<Vec<Row>, Error>{
    let rows = client.query(fs::read_to_string("queries/insert_url.sql")?, &[table_name, url, tinified_url]).await?;
    rows
}

pub fn delete_url_mapping(table_name: &str, url: Option<&str>, detinified_url: Option<&str>) {
    todo!()
}

