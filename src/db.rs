use crate::data::{Book, BookRequest};
use crate::{error, error::Error::*, DBCon, DBPool};
use mobc::Pool;
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use std::fs;
use std::str::FromStr;
use std::time::Duration;
use tokio_postgres::{Config, Error, NoTls, Row};

type Result<T> = std::result::Result<T, error::Error>;

const DB_POOL_MAX_OPEN: u64 = 32;
const DB_POOL_MAX_IDLE: u64 = 8;
const DB_POOL_TIMEOUT_SECONDS: u64 = 15;
const INIT_SQL: &str = "./db.sql";
const TABLE: &str = "book";
const SELECT_FIELDS: &str = "id, title, author";
const DB_URL: &str = "postgres://postgres:postgres@localhost:5432/postgres";

pub async fn init_db(db_pool: &DBPool) -> Result<()> {
    let init_file = fs::read_to_string(INIT_SQL)?;
    let con = get_db_con(db_pool).await?;
    con.batch_execute(init_file.as_str())
        .await
        .map_err(DBInitError)?;
    Ok(())
}

pub async fn get_db_con(db_pool: &DBPool) -> Result<DBCon> {
    db_pool.get().await.map_err(DBPoolError)
}

pub fn create_pool() -> std::result::Result<DBPool, mobc::Error<Error>> {
    let config = Config::from_str(DB_URL)?;

    let manager = PgConnectionManager::new(config, NoTls);
    Ok(Pool::builder()
        .max_open(DB_POOL_MAX_OPEN)
        .max_idle(DB_POOL_MAX_IDLE)
        .get_timeout(Some(Duration::from_secs(DB_POOL_TIMEOUT_SECONDS)))
        .build(manager))
}

pub async fn fetch_books(db_pool: &DBPool) -> Result<Vec<Book>> {
    let con = get_db_con(db_pool).await?;
    let query = format!("SELECT {} FROM {} ORDER BY id", SELECT_FIELDS, TABLE);
    let q = con.query(query.as_str(), &[]).await;
    let rows = q.map_err(DBQueryError)?;

    Ok(rows.iter().map(|r| row_to_book(&r)).collect())
}

pub async fn fetch_book(db_pool: &DBPool, id: i32) -> Result<Book> {
    let con = get_db_con(db_pool).await?;
    let where_clause = "WHERE id = $1";
    let query = format!("SELECT {} FROM {} {}", SELECT_FIELDS, TABLE, where_clause);
    let q = con.query_one(query.as_str(), &[&id]).await;
    let row = q.map_err(DBQueryError)?;

    Ok(row_to_book(&row))
}

pub async fn create_book(db_pool: &DBPool, body: BookRequest) -> Result<Book> {
    let con = get_db_con(db_pool).await?;
    let query = format!(
        "INSERT INTO {} (title, author) VALUES ($1, $2) RETURNING *",
        TABLE
    );
    let row = con
        .query_one(query.as_str(), &[&body.title, &body.author])
        .await
        .map_err(DBQueryError)?;
    Ok(row_to_book(&row))
}

pub async fn update_book(db_pool: &DBPool, id: i32, body: BookRequest) -> Result<Book> {
    let con = get_db_con(db_pool).await?;
    let query = format!(
        "UPDATE {} SET title = $1, author = $2 WHERE id = $3 RETURNING *",
        TABLE
    );
    let row = con
        .query_one(query.as_str(), &[&body.title, &body.author, &id])
        .await
        .map_err(DBQueryError)?;
    Ok(row_to_book(&row))
}

pub async fn delete_book(db_pool: &DBPool, id: i32) -> Result<u64> {
    let con = get_db_con(db_pool).await?;
    let query = format!("DELETE FROM {} WHERE id = $1", TABLE);
    con.execute(query.as_str(), &[&id])
        .await
        .map_err(DBQueryError)
}

fn row_to_book(row: &Row) -> Book {
    let id: i32 = row.get(0);
    let title: String = row.get(1);
    let author: String = row.get(2);
    Book { id, title, author }
}
