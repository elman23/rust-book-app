use crate::data::{BookRequest, BookResponse};
use crate::{db, error::Error::*, DBPool, Result};
use warp::{http::StatusCode, reject, reply::json, Reply};

pub async fn health_handler(db_pool: DBPool) -> Result<impl Reply> {
    let db = db::get_db_con(&db_pool)
        .await
        .map_err(|e| reject::custom(e))?;
    db.execute("SELECT 1", &[])
        .await
        .map_err(|e| reject::custom(DBQueryError(e)))?;
    Ok(StatusCode::OK)
}

pub async fn list_books_handler(uid: String, db_pool: DBPool) -> Result<impl Reply> {
    let books = db::fetch_books(&db_pool)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(json::<Vec<_>>(
        &books.into_iter().map(|t| BookResponse::of(t)).collect(),
    ))
}

pub async fn list_book_handler(uid: String, id: i32, db_pool: DBPool) -> Result<impl Reply> {
    let book = db::fetch_book(&db_pool, id)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(json(&BookResponse::of(book)))
}

pub async fn create_book_handler(
    uid: String,
    body: BookRequest,
    db_pool: DBPool,
) -> Result<impl Reply> {
    Ok(json(&BookResponse::of(
        db::create_book(&db_pool, body)
            .await
            .map_err(|e| reject::custom(e))?,
    )))
}

pub async fn update_book_handler(
    uid: String,
    id: i32,
    body: BookRequest,
    db_pool: DBPool,
) -> Result<impl Reply> {
    Ok(json(&BookResponse::of(
        db::update_book(&db_pool, id, body)
            .await
            .map_err(|e| reject::custom(e))?,
    )))
}

pub async fn delete_book_handler(uid: String, id: i32, db_pool: DBPool) -> Result<impl Reply> {
    db::delete_book(&db_pool, id)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(StatusCode::OK)
}
