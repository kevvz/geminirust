use reqwest::header::{CONTENT_TYPE,ACCEPT,HeaderMap};
use std::collections::HashMap;
use geminirust::client::Gem;

use serde::{Serialize,Deserialize};
use serde_json::Result;

#[tokio::main]
async fn main() -> Result<()>{
    let client = Gem::new();
    


 //   println!("Hello world");
    let s = client.generate_content(String::from("Who are you?")).await;
  
    println!("{}",&s.unwrap());

    Ok(())


     
}