use actix_web::{get, App, HttpResponse, HttpServer, Responder};
//import the random fruit function from the lib.rs file
use distroless_web::random_fruit;

//create a function that returns a hello world
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World Random Fruit!")
}

//create a function that returns a random fruit
#[get("/fruit")]
async fn fruit() -> impl Responder {
    //print the random fruit
    println!("Random Fruit: {}", random_fruit());
    HttpResponse::Ok().body(random_fruit())
}

//create a function that returns a 200 status code
#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok()
}

//create a function that returns the version of the service
#[get("/version")]
async fn version() -> impl Responder {
    //print the version of the service
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    HttpResponse::Ok().body(env!("CARGO_PKG_VERSION"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //add a print message to the console that the service is running
    println!("Running the service");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(fruit)
            .service(health)
            .service(version)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
