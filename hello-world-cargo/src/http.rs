extern crate reqwest;

pub mod http {
    pub async fn http_server() {
        match reqwest::get("https://www.facebook.com/").await {
            Ok(response) => {
                if response.status() == reqwest::StatusCode::OK {
                    match response.text().await {
                        Ok(text) => {
                            println!("Response Text: {}", text);
                        }
                        Err(e) => {
                            println!("Error: {}", e);
                        }
                    }
                } else {
                    println!("Error: {}", response.status());
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
