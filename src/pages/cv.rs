use actix_files::NamedFile;
use actix_web::{
    get,
    error::ErrorBadRequest,
    http::header::{ContentDisposition, DispositionType},
    HttpRequest, Responder,
};

#[get("/cv")]
async fn file(req: HttpRequest) -> impl Responder {
    let pdf_file_path = "./adrian_herda_cv.pdf";
    match NamedFile::open(&pdf_file_path) {
        Ok(file) => file
            .set_content_disposition(ContentDisposition {
                disposition: DispositionType::Inline,
                parameters: Vec::new(),
            })
            .into_response(&req),
        Err(_) => {
            return ErrorBadRequest("File not Found").into();
        }
    }
}
