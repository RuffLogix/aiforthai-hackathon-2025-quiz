use ntex::web;

const ADDRESS: &str = "0.0.0.0";
const PORT: u16 = 8081;

#[web::get("/")]
async fn hello() -> impl web::Responder {
    println!("[API 2] Calling hello()");
    web::HttpResponse::Ok().body("Hello world from API 2")    
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
