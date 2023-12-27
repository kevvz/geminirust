use crate::configs::UrlConfig;
use serde::{Serialize,Deserialize};
use serde_json::{Result,Value};


#[derive(Serialize, Deserialize)]
pub struct Content {
    contents: Parts,
}
#[derive(Serialize, Deserialize)]
pub struct Parts {
    parts: Text,
}
#[derive(Serialize, Deserialize)]
pub struct Text {
    text: String,
}

pub struct Gem {
    client: reqwest::Client,
    model_name: String,
    url: UrlConfig,
}

impl Gem {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            model_name: String::from("gemini-pro"),
            url: UrlConfig::new(),
        }
    }

    pub fn change_model_name(&mut self, name: String){
        self.model_name = name;
        
    }

    pub fn print_params(&self) -> String {
        let result = String::from(&self.model_name);
     //   println!("Hello world");
        result
    }

    pub async fn generate_content(&self,input: String)->Result<String> {
        
        let input_ = Content {
           contents: Parts {
            parts: Text {
                text: input,
            },
           },
        };

        let request = self.client.post(self.url.url())
            .headers(self.url.headers())
            .json(&input_)           
            .send()
            .await
            .unwrap()
            .text()
            .await;
        
        // let result = match request {
        //     Ok(request) => request,
        //     Err(request) => return Err(Error),
        // };
        
        let v: Value = serde_json::from_str(&request.unwrap())?;
        
      //  println!("{}",v["candidates"][0]["content"]["parts"][0]["text"]);

        Ok(v["candidates"][0]["content"]["parts"][0]["text"].to_string())
    }

}

