use actix_files::Files;
use actix_web::{web::Redirect, App, HttpServer};

pub mod components;
mod data;
mod helpers;
mod pages;

use pages::{about, contact, home, projects, visuals, cv};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/static", "./static"))
            .service(cv::file)
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
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}
