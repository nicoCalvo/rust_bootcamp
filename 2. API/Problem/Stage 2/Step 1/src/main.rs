#[macro_use]
extern crate rocket;

use std::env;

mod cors;
mod handlers;
mod models;

use cors::*;
use dotenvy::dotenv;
use handlers::*;
use sqlx::{postgres::PgPoolOptions, Row};

extern crate pretty_env_logger;




use crate::models::Question;

#[macro_use] extern crate log;

static MAX_CON: u32 = 5;

#[launch]
async fn rocket() -> _ {
    pretty_env_logger::init();
    dotenv().ok();

    for (key, value) in env::vars() {
        info!("{}: {}", key, value);
    }
    let db_url = env::var("DATABASE_URL").expect("A database url must be specified in .env");
    let pool = PgPoolOptions::new()
        .max_connections(MAX_CON)
        .connect(&db_url).await.unwrap();

    let recs = sqlx::query!("SELECT * FROM questions")
        .fetch_all(&pool).await.expect("DB not ready");
    info!("********* Question Records *********");
    info!("{:?}", recs);

    rocket::build()
        .mount(
            "/",
            routes![
                create_question,
                read_questions,
                delete_question,
                create_answer,
                read_answers,
                delete_answer
            ],
        )
        .attach(CORS)
}
