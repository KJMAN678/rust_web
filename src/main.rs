use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Error, error};
use tera::Tera;

#[get("/")]
async fn hello(tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {

    let mut ctx = tera::Context::new();
    let view = tmpl.render("index.html", &ctx)
            .map_err(|e| error::ErrorInternalServerError(e))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {

        let templates = Tera::new("templates/**/*").unwrap();
        App::new()
            .data(templates)
            .service(hello)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}