use std::error::Error;

use openssl::ssl::{SslConnector, SslMethod};
use postgres::{Client, NoTls};
use postgres_openssl::MakeTlsConnector;

pub struct CreateConnection {
    pub conn_str: String,
    pub is_ssl: bool,
}

pub fn create_connection(config: CreateConnection) -> Result<Client, Box<dyn Error>> {
    let client = match config.is_ssl {
        true => {
            let builder = SslConnector::builder(SslMethod::tls())?;
            let _connector = MakeTlsConnector::new(builder.build());
            Client::connect(&config.conn_str, NoTls)?
        }
        false => Client::connect(&config.conn_str, NoTls)?,
    };

    Ok(client)
}
