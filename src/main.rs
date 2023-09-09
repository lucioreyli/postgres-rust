use openssl::ssl::{SslConnector, SslMethod};
use postgres::Client;
use postgres_openssl::MakeTlsConnector;

const CON_STR: &str = "postgres://postgres:abacaxi@localhost:5432/postgres";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let builder = SslConnector::builder(SslMethod::tls())?;
    let connector = MakeTlsConnector::new(builder.build());
    let mut client = Client::connect(CON_STR, connector)?;

    let result = client.query(
        "
SELECT tablename FROM pg_tables WHERE schemaname = 'public';
        ",
        &[],
    )?;

    for row in result {
        let name: &str = row.get(0);

        println!("{}", name);
    }

    let _ = client.close();

    Ok(())
}
