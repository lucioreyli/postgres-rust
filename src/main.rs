mod create_connection;

const CON_STR: &str = "postgres://postgres:abacaxi@localhost:5432/postgres";

fn main() -> Result<(), String> {
    let connection_config = create_connection::CreateConnection {
        conn_str: CON_STR.to_string(),
        is_ssl: false,
    };
    let result_client = create_connection::create_connection(connection_config);

    if result_client.is_err() {
        let err = result_client.err().unwrap();
        let err_msg = format!("{}", err);
        return Err(err_msg);
    }

    let mut client = result_client.unwrap();

    let query_result = client.query(
        "
        CREATE TABLE users_dam (
            id SERIAL PRIMARY KEY,
            name TEXT,
            email VARCHAR(250) UNIQUE
        );
        ",
        &[],
    );

    let query_result = client.query(
        "
SELECT tablename FROM pg_tables WHERE;
        ",
        &[],
    );

    if query_result.is_err() {
        let err = query_result.err().unwrap();
        let err_msg = format!("{}", err);
        return Err(err_msg);
    }

    let query = query_result.unwrap();

    let response: Vec<String> = query.into_iter().map(|row| row.get(0)).collect();

    let _ = client.close();

    println!("finish");
    Ok(())
}
