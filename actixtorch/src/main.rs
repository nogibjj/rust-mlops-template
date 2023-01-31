/*Build a actix microservice front end for PyTorch*/

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
//import the random fruit function from the lib.rs file
use actixtorch::predict;

//create a function that returns a hello world
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World PyTorch")
}

//create a function that runs lib.rs function to predict an image from disk using model on disk
#[get("/predict")]
async fn predictit() -> impl Responder {
    //print the random fruit
    println!("Creating PyTorch predict");
    //run the predict function from lib.rs
    let prediction = predict();
    //if the prediction is ok return the "yes" string
    if prediction.is_ok() {
        HttpResponse::Ok().body("Yes")
    } else {
        //if the prediction is not ok return the "no" string
        HttpResponse::Ok().body("No")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //add a print message to the console that the service is running
    println!("Running the service");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(predictit)

    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

