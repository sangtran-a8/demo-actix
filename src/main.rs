use actix_web::get;
use actix_web::web::Query;
use actix_web::{App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct AddParams {
    numbers: String,
}

#[derive(Serialize)]
struct AddResponse {
    numbers: Vec<f64>,
    sum: f64,
    message: String,
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[get("/sum")]
async fn sum(Query(params): Query<AddParams>) -> impl Responder {
    // Split the 'numbers' string by commas, parse each piece into a f64, and collect into a Vec<f64>
    let numbers: Vec<f64> = params
        .numbers
        .split(',')
        .filter_map(|n| n.parse().ok())
        .collect();

    // Sum all the numbers
    let sum: f64 = numbers.iter().sum();

    // Build the response structure
    let response = AddResponse {
        numbers: numbers.clone(),
        sum,
        message: format!("The sum of {:?} is {}", numbers, sum),
    };

    // A structured JSON should be returned
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(health).service(sum))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
