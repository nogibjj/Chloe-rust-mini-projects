/*An actix Microservice that has multiple routes:
A.  / that turns a hello world
B. /fruit that returns a random fruit
C. /health that returns a 200 status code
D. /version that returns the version of the service
*/

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
//import the random fruit function from the lib.rs file
use webdocker::random_fruit;
use webdocker::random_drink;

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
    let message = format!("Today's fruit recommendation: {}", random_fruit());
    HttpResponse::Ok().body(message)
}

#[get("/drink")]
async fn drink() -> impl Responder {
    //print the random drink
    println!("Random Drink: {}", random_drink());
    let message = format!("Today's drink recommendation: {}", random_drink());
    HttpResponse::Ok().body(message)
}

#[get("/lunchCombo")]
async fn lunchCombo() -> impl Responder {
    //print the random fruit and drink combination
    println!(
        "Random Fruit: {}, Randome Drink: {}",
        random_fruit(),
        random_drink()
    );
    let message = format!(
        "Today's lunch recommendation: {} and {}",
        random_fruit(),
        random_drink()
    );
    HttpResponse::Ok().body(message)
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
            .service(drink)
            .service(lunchCombo)
            .service(health)
            .service(version)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
