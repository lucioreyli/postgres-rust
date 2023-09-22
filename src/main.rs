use postgres::Client;

mod create_connection;

fn main() -> Result<String, String> {
    let result_client = create_connection::create_connection();
    let mut client: Client;
    match result_client {
        Ok(value) => {
            client = value;
            return Ok("".into());
        }
        Err(e) => Err("deu ruimkk".into()),
    };

    Ok("hehe".into())
}
