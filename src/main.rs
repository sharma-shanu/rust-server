use actix_web :: {get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn health(req:HttpRequest)->impl Responder{
    println!("api hit {:?}",req);
    HttpResponse::Ok().body("Healthy")
}

#[get("/hello")]
async fn hello()->impl Responder{
    HttpResponse::Ok().body("Hello There")
}

#[actix_web::main]
async fn main()-> std::io::Result<()>{
    HttpServer::new(||{
        App::new()
        .service(hello)
        .route("/health", web::get().to(health))
    }).bind(("127.0.0.1", 3000))?
    .run()
    .await
}