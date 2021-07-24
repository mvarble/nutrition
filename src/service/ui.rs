use actix_files::NamedFile;
use actix_web::{web, HttpRequest, HttpResponse, Result};
use std::path::PathBuf;

use super::AppState;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(home)))
        .service(web::resource("/index.html").route(web::get().to(redirect)))
        .service(web::resource("/{filename:.*}").route(web::get().to(ui)));
}

async fn home(data: web::Data<AppState>) -> Result<NamedFile> {
    println!("HI THERE");
    let path = data.ui_dir.join("index.html");
    serve_file(path).await
}

async fn redirect() -> HttpResponse {
    HttpResponse::PermanentRedirect()
        .header("Location", "/")
        .finish()
}

async fn ui(req: HttpRequest, data: web::Data<AppState>) -> Result<NamedFile> {
    let path = data.ui_dir.join(req.match_info().query("filename"));
    serve_file(path).await
}

async fn serve_file(path: PathBuf) -> Result<NamedFile> {
    println!("{:#?}", path);
    Ok(NamedFile::open(path)?)
}
