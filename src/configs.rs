use reqwest::header::{HeaderMap};
use reqwest::header::{ACCEPT,CONTENT_TYPE};


pub const API_KEY: &str = "";

pub struct UrlConfig {

    url_: String,
    key: String,
    model: String,


}

impl UrlConfig {
    pub fn new() ->Self {
        Self {
            url_: String::from("https://generativelanguage.googleapis.com/v1beta/models/"),
            key: API_KEY.to_owned(),
            model: String::from("gemini-pro"),
        }
    }
    
    pub fn url(&self)-> String {
     let result = &mut self.url_.to_string();
     result.push_str(&self.model);
     result.push_str(&mut String::from(":generateContent?key="));
     result.push_str(&self.key);
     result.to_string()
    }


    pub fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE,"application/json".parse().unwrap());
        headers.insert(ACCEPT,"application/json".parse().unwrap());
        headers
    }   

}


pub struct GenerationConfig {

    

}