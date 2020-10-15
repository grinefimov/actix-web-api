use rusqlite::params;

mod app;
mod db;

// TODO: authenticate(&state, &req)

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    const DB_PATH: &str = "aws.db";
    initialize_db(DB_PATH);

    std::env::set_var("RUST_LOG", "actix_web=info"); //my_errors=debug,
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    app::start().await
}

fn initialize_db(db_path: &str) {
    rusqlite::Connection::open(db_path)
        .unwrap()
        .execute(
            "CREATE TABLE IF NOT EXISTS Products
             (
                 ID     integer primary key,
                 Name   text not null
             );",
            params![],
        )
        .unwrap();
}
