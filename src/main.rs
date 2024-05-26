use mobc::{Connection, Pool};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use std::convert::Infallible;
use tokio_postgres::NoTls;
use warp::{Filter, Rejection};

mod data;
mod db;
mod error;
mod handler;

type Result<T> = std::result::Result<T, Rejection>;
type DBCon = Connection<PgConnectionManager<NoTls>>;
type DBPool = Pool<PgConnectionManager<NoTls>>;

#[tokio::main]
async fn main() {
    let db_pool = db::create_pool().expect("database pool can be created");

    db::init_db(&db_pool)
        .await
        .expect("database can be initialized");

    let health_route = warp::path!("healthcheck")
        .and(with_db(db_pool.clone()))
        .and_then(handler::health_handler);

    let book = warp::path("book");
    let book_routes = book
        .and(warp::get())
        .and(warp::path::param())
        .and(with_db(db_pool.clone()))
        .and_then(handler::list_book_handler)
        .or(book
            .and(warp::get())
            //.and(warp::path::param())
            .and(with_db(db_pool.clone()))
            .and_then(handler::list_books_handler))
        .or(book
            .and(warp::post())
            .and(warp::body::json())
            .and(with_db(db_pool.clone()))
            .and_then(handler::create_book_handler))
        .or(book
            .and(warp::put())
            .and(warp::path::param())
            .and(warp::body::json())
            .and(with_db(db_pool.clone()))
            .and_then(handler::update_book_handler))
        .or(book
            .and(warp::delete())
            .and(warp::path::param())
            .and(with_db(db_pool.clone()))
            .and_then(handler::delete_book_handler));

    let routes = health_route
        .or(book_routes)
        .with(warp::cors().allow_any_origin())
        .recover(error::handle_rejection);

    println!("🚀 Server started successfully!");
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

fn with_db(db_pool: DBPool) -> impl Filter<Extract = (DBPool,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}
