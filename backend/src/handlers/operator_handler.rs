use actix_web::{web, HttpResponse, Responder};
use crate::cache;
use crate::models::operator::{Operator, HellaOperator};

// GET /api/operators
pub async fn get_operators() -> impl Responder {
    if let Some(operators) = cache::get_operators().await {
        return HttpResponse::Ok().json(operators);
    }

    match fetch_operators_from_api().await {
        Ok(operators) => {
            cache::set_operators(operators.clone()).await;
            HttpResponse::Ok().json(operators)
        }
        Err(e) => {
            eprintln!("Error fetching operators: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch data")
        }
    }
}

pub async fn fetch_operators_from_api() -> Result<Vec<Operator>, Box<dyn std::error::Error>> {
    let url = "https://awedtan.ca/api/operator";

    println!("Fetching from HellaAPI...");
    
    let client = reqwest::Client::builder()
        .build()?;
    
    let response = client.get(url).send().await?;
    
    println!("Response status: {}", response.status());
    
    let text = response.text().await?;
    
    println!("Response length: {} bytes ({} MB)", text.len(), text.len() / 1024 / 1024);
    
    let hella_operators: Vec<HellaOperator> = serde_json::from_str(&text)?;
    
    println!("Parsed {} operators", hella_operators.len());
    
    let operators: Vec<Operator> = hella_operators
        .into_iter()
        .map(Operator::from)
        .collect();

    println!("Converted to {} operators", operators.len());

    Ok(operators)
}