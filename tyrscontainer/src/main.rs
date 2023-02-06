/*A Microservice for processing strings */

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
//import the reverse, pig_latin, and binary functions from the lib.rs file
use tyrscontainer::{binary, pig_latin, reverse};

//create a function that returns a hello world message and explains what the service does
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World! This is a Microservice for processing strings. You can use the following routes to process strings: /reverse, /piglatin, /binary")
}

//create routes for the reverse, pig_latin, and binary functions
#[get("/reverse/{input}")]
async fn reverse_it(input: actix_web::web::Path<String>) -> impl Responder {
    //print the reverse of the input
    println!("Reverse: {}", reverse(&input));
    HttpResponse::Ok().body(reverse(&input))
}

#[get("/piglatin/{input}")]
async fn piglatin_it(input: actix_web::web::Path<String>) -> impl Responder {
    //print the pig latin of the input
    println!("Pig Latin: {}", pig_latin(&input));
    HttpResponse::Ok().body(pig_latin(&input))
}

#[get("/binary/{input}")]
async fn binary_it(input: actix_web::web::Path<String>) -> impl Responder {
    //print the binary of the input
    println!("Binary: {}", binary(&input));
    HttpResponse::Ok().body(binary(&input))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //add a print message to the console that the service is running
    println!("Running the service");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(reverse_it)
            .service(piglatin_it)
            .service(binary_it)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
