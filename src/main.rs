use actix_web::{web::Redirect, App, HttpServer, HttpResponse};
use actix_files::Files;

mod pages;
use pages::{home, about, projects};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/static", "./static"))
            .service(home::home)
            .service(about::about)
            .service(projects::projects)
            .service(Redirect::new("/home", "/"))
            .service(Redirect::new("/index", "/"))
            //.default_service(index::index)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
