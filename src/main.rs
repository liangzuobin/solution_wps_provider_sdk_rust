mod api;
mod service;
mod error;
mod model;

use crate::api::preview::fetch_file;

use actix_web::{
    error::ResponseError,
    get,
    http::{header::ContentType, StatusCode},
    middleware::Logger,
    web::Json,
    App, HttpResponse, HttpServer,
};
use serde::{Deserialize, Serialize};
use strum::Display;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move || {
        // init service here
        let logger = Logger::default();

        App::new()
            .wrap(logger)
            // .app_data(service)
            .service(get_user)
            .service(fetch_file)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


// 以下是自己写的调试用的代码，commit 之前要删除掉，没用的

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: String,
    name: String,
}

#[derive(Debug, Display)]
enum Error {}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }
}

#[get("/user")]
async fn get_user() -> Result<Json<User>, Error> {
    return Result::Ok(Json(User {
        id: "1".to_string(),
        name: "foo".to_string(),
    }));
}
