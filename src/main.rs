use actix_web::http::header::ContentType;
use actix_web::web::Redirect;
use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use gas_backend::common;
use std::fs;

#[get("/")]
async fn index() -> impl Responder {
    Redirect::to("/html/index")
}

#[get("/html/{page}")]
async fn page(page: web::Path<String>) -> impl Responder {
    #[cfg(debug_assertions)]
    println!("request page: {page}");

    if let Ok(content) = fs::read_to_string(format!("html/{page}.html")) {
        HttpResponse::Ok()
            .content_type(ContentType::html())
            .body(common(content))
    } else {
        Redirect::to("/404")
    }
}

#[get("/assets/{path:.*}")]
async fn assets_data(path: web::Path<String>) -> impl Responder {
    if let Ok(asset) = fs::read(format!("assets/{path}")) {
        let mut response = HttpResponse::Ok();

        if path.ends_with("jpg") {
            response.content_type(ContentType::jpeg());
        } else if path.ends_with("png") {
            response.content_type(ContentType::png());
        }

        response.body(asset)
    } else {
        HttpResponse::NotFound()
    }
}

#[get("/404")]
async fn not_found() -> impl Responder {
    "404 - Content not found"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[cfg(debug_assertions)]
    let address = "127.0.0.1";
    #[cfg(not(debug_assertions))]
    let address = "0.0.0.0";

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .service(index)
            .service(assets_data)
            .service(page)
            .service(not_found)
    })
    .bind((address, 8080))?
    .run()
    .await
}
