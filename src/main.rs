use reqwest::{Error, Client};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if both a verb and URL have been provided
    if args.len() < 3 {
        println!("Usage: cargo run <VERB> <URL>");
        std::process::exit(1);
    }

    let verb = &args[1].to_uppercase();
    let url = &args[2];

    // Create a new Client
    let client = Client::new();

    // Match the HTTP verb and perform the request accordingly
    let res = match verb.as_str() {
        "GET" => client.get(url).send().await?,
        "POST" => client.post(url).send().await?, // For POST, consider modifying to include body data if needed
        "PUT" => client.put(url).send().await?,
        "DELETE" => client.delete(url).send().await?,
        // Add more cases for other HTTP methods as needed
        _ => {
            println!("Unsupported HTTP verb: {}", verb);
            std::process::exit(1);
        }
    };

    // Print the status code of the response
    println!("Status: {}", res.status());

    // Get the response text
    let body = res.text().await?;

    // Print the response body
    println!("Body:\n{}", body);

    Ok(())
}