use actix_files::Files;
use actix_web::{web::Redirect, App, HttpServer};

mod components;
mod helpers;
mod pages;

use pages::{about, contact, home, projects, visuals};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/static", "./static"))
            .service(home::home)
            .service(about::about)
            .service(projects::projects)
            .service(projects::projects_json)
            .service(visuals::visuals)
            .service(visuals::visual)
            .service(contact::contact)
            .service(Redirect::new("/home", "/"))
            .service(Redirect::new("/index", "/"))
        //.default_service(index::index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
