use actix_web::{get, web, App, HttpResponse, HttpServer, Error, error};
use tera::Tera;
use std::env;

#[get("/")]
async fn hello(tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {

    let ctx = tera::Context::new();
    let view = tmpl.render("index.html", &ctx)
            .map_err(|e| error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    HttpServer::new(|| {

        let templates = Tera::new("templates/**/*").unwrap();
        App::new()
            .data(templates)
            .service(hello)
    })
    .bind(("0.0.0.0", port))
    .expect("Can not bind to port 8000")
    .run()
    .await
}