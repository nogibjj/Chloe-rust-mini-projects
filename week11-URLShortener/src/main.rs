use actix_web::{error::Error, web, App, HttpResponse, HttpServer, Responder};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(serde::Deserialize)]
struct UrlData {
    url: String,
}

struct AppState {
    urls: Mutex<HashMap<String, String>>,
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn shorten(
    url: web::Form<UrlData>,
    data: web::Data<Arc<AppState>>,
) -> Result<HttpResponse, Error> {
    let mut rng = thread_rng();
    let slug: String = std::iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(6)
        .map(char::from)
        .collect();

    let url = url.url.clone();

    let mut map = data.urls.lock().unwrap();
    map.insert(slug.clone(), url.clone());

    let short_url = format!("http://localhost:8080/{}", slug);

    Ok(HttpResponse::Ok().body(short_url))
}

async fn redirect(
    path: web::Path<String>,
    data: web::Data<Arc<AppState>>,
) -> Result<HttpResponse, Error> {
    match data.urls.lock().unwrap().get(&path.into_inner()) {
        Some(url) => Ok(HttpResponse::Found()
            .append_header(("Location", url.clone()))
            .finish()),

        None => Err(actix_web::error::ErrorNotFound("URL not found")),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = Arc::new(AppState {
        urls: Mutex::new(HashMap::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/", web::get().to(index))
            .route("/shorten", web::post().to(shorten))
            .data(data.clone())
            .route("/{slug}", web::get().to(redirect))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
