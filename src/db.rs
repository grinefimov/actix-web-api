use rusqlite::params;
use serde::{Deserialize, Serialize};

pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;

pub fn select_products(conn: Connection) -> Result<Vec<Product>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT ID, Name
    FROM Products;",
    )?;

    stmt.query_map(params![], |row| Ok(Product::from_row(row)))
        .and_then(Iterator::collect)
}

// TODO: Export Models mod

#[derive(Serialize, Deserialize)]
pub struct Product {
    pub id: Option<u32>,
    pub name: Option<String>,
}

impl Product {
    fn from_row(row: &rusqlite::Row) -> Product {
        Product {
            id: row.get(0).ok().unwrap(),
            name: row.get(1).ok().unwrap(),
        }
    }
}
