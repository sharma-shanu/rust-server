use actix_web :: {delete,web::Path, get, post, put, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::{Deserialize,Serialize};

#[derive(Deserialize,Serialize)]
pub struct CreateBookingUrl{
    pub id:String,
}

async fn health(_req:HttpRequest)->impl Responder{
    println!("api hit {:?}",_req);
    HttpResponse::Ok().body("Healthy")
}

#[get("/hello")]
async fn hello()->impl Responder{
    HttpResponse::Ok().body("Hello")
}

#[get("/bookings")]
async fn get_all_bookings(_req:HttpRequest)->impl Responder{
    HttpResponse::Ok().body("all-bookings")
}

#[post("/booking")]
async fn create_booking(_req:HttpRequest)->impl Responder{
    HttpResponse::Created().body("created")
}

#[get("/booking/{id}")]
async fn get_booking(path:Path<CreateBookingUrl>)->impl Responder{
    let id: String = path.into_inner().id;
    HttpResponse::Ok().body(id)
}

#[put("/booking")]
async fn update_booking(_req:HttpRequest)->impl Responder{
    HttpResponse::Ok().body("updated the entry")
}

#[delete("/booking")]
async fn delete_booking(_req:HttpRequest)->impl Responder{
    HttpResponse::Ok().body("deleted")
}

/// #[patch("/booking")]

#[actix_web::main]
async fn main()-> std::io::Result<()>{
    HttpServer::new(||{
        App::new()
        .service(hello)
        .service(create_booking)
        .service(get_booking)
        .service(get_all_bookings)
        .service(update_booking)
        .service(delete_booking)
        .route("/health", web::get().to(health))
    }).bind(("127.0.0.1", 3000))?
    .run()
    .await
}