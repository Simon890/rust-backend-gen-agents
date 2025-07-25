use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpServer, Responder, HttpResponse};
use reqwest::Client as HttpClient;
use serde_json::Value;
use std::sync::Arc;

struct AppState {
    client: HttpClient,
}

async fn fetch_and_respond(url: &str, data: &web::Data<Arc<AppState>>) -> HttpResponse {
    match data.client.get(url).send().await {
        Ok(resp) => match resp.json::<Value>().await {
            Ok(json) => HttpResponse::Ok().json(json),
            Err(_) => HttpResponse::InternalServerError().body("Failed to parse JSON"),
        },
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch data"),
    }
}

async fn get_all_characters(data: web::Data<Arc<AppState>>) -> impl Responder {
    fetch_and_respond("https://hp-api.onrender.com/api/characters", &data).await
}

async fn get_students(data: web::Data<Arc<AppState>>) -> impl Responder {
    fetch_and_respond("https://hp-api.onrender.com/api/characters/students", &data).await
}

async fn get_staff(data: web::Data<Arc<AppState>>) -> impl Responder {
    fetch_and_respond("https://hp-api.onrender.com/api/characters/staff", &data).await
}

async fn get_character_by_name(
    data: web::Data<Arc<AppState>>,
    name: web::Path<String>,
) -> impl Responder {
    let name_ref = name.into_inner();
    let url = "https://hp-api.onrender.com/api/characters";
    match data.client.get(url).send().await {
        Ok(resp) => match resp.json::<Vec<Value>>().await {
            Ok(list) => {
                let filtered: Vec<Value> = list
                    .into_iter()
                    .filter(|c| {
                        c.get("name")
                            .and_then(Value::as_str)
                            .map(|n| n.eq_ignore_ascii_case(&name_ref))
                            .unwrap_or(false)
                    })
                    .collect();
                if !filtered.is_empty() {
                    HttpResponse::Ok().json(filtered)
                } else {
                    HttpResponse::NotFound().body("Character not found")
                }
            }
            Err(_) => HttpResponse::InternalServerError().body("Failed to parse JSON"),
        },
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch characters"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = HttpClient::new();
    let state = Arc::new(AppState { client });

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::permissive()
                    .allowed_origin_fn(|origin, _| {
                        origin.as_bytes().starts_with(b"http://localhost") || origin == "null"
                    })
                    .allowed_methods(vec!["GET"])
                    .allowed_headers(vec![header::ACCEPT, header::CONTENT_TYPE])
                    .max_age(3600),
            )
            .app_data(web::Data::new(state.clone()))
            .route("/characters", web::get().to(get_all_characters))
            .route("/characters/students", web::get().to(get_students))
            .route("/characters/staff", web::get().to(get_staff))
            .route("/characters/{name}", web::get().to(get_character_by_name))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}