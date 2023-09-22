use std::error::Error;

use openssl::ssl::{SslConnector, SslMethod};
use postgres::{Client, NoTls};
use postgres_openssl::MakeTlsConnector;

const CON_STR: &str = "postgres://postgres:abacaxi@localhost:5432/postgres";

pub fn create_connection() -> Result<Client, Box<dyn Error>> {
    let builder = match SslConnector::builder(SslMethod::tls()) {
        Err(e) => panic!("{:?}", e),
        Ok(_builder) => _builder,
    };
    let _connector = MakeTlsConnector::new(builder.build());
    let client = Client::connect(CON_STR, NoTls)?;

    return Ok(client);
}
