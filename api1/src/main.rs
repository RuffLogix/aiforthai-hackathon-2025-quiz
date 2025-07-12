use anyhow::Result;
use ntex::web;

const ADDRESS: &str = "0.0.0.0";
const API_ADDRESS: &str = "http://api2:8081";
const PORT: u16 = 8080;

async fn call_api() -> Result<String> {
    println!("Hello");
    let client = reqwest::Client::new();
    let response = client
        .get(API_ADDRESS)
        .send()
        .await?;

    let body = response.text().await?;

    Ok(body)
}

#[web::get("/")]
async fn hello() -> impl web::Responder {
    println!("[API 1] Calling hello()");
    match call_api().await {
        Ok(val) => println!("[RESPONSE API 2] {}", val),
        Err(err) => println!("[ERROR API 2] {}", err)
    }
    web::HttpResponse::Ok().body("Hello world from API 1")    
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| {
        web::App::new()
            .service(hello)
    })    
    .bind((ADDRESS, PORT))?
    .run()
    .await
}
