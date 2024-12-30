use actix_web::{get, web, App, HttpServer, Responder};

use website::contents::{DataList, DataEntry, Date};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(hello)
            .service(greet)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[get("/")]
async fn index() -> impl Responder {
    "<h1>This is Adrian Herda's Actix Web server!\nWelcome!</h1>"
}

#[get("/hello")]
async fn hello() -> impl Responder {
    "Hello world!"
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

#[get("/tatarozpiska")]
async fn tata_rozpiska() -> impl Responder {
    let data = DataList::new(
        "Tata Rozpiska".to_string(),
        vec![
            DataEntry::new(
                1,
                "Zakupy".to_string(),
                Some(Date::new(1, 1, Some(2021))),
                100.0,
            ),
            DataEntry::new(
                2,
                "Zakupy".to_string(),
                Some(Date::new(2,1,Some(2021))),
                200.0,
            ),
            DataEntry::new(
                3,
                "Zakupy".to_string(),
                None,
                300.0,
            ),
        ],
    );
    format!("{:#?}", data.get_data())
}

#[post("/tatarozpiska/add/{name}-{id}-{amount}")]
async fn tata_rozpiska_add(
    web::Path((name, id, amount)): web::Path<(String, u64, f64)>,
) -> impl Responder {
    format!("Added: {} {} {}", name, id, amount)
}
