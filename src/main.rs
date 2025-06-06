use std::io;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct API_Response {
    contents: Contents,
}

#[derive(Debug, Deserialize)]
struct Contents {
    translated: String,
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    println!("This API only lets you call 10 times an hour, so use them wisely! :)");

    loop {
        println!("Input sentence (or exit to quit):");
        let mut sentence = String::new();

        io::stdin()
            .read_line(&mut sentence)
            .expect("Failed to read line");

        if sentence.trim().eq_ignore_ascii_case("exit") {
            break;
        }

        let mut url: String = "https://api.funtranslations.com/translate/yoda.json?text="
            .to_owned();

        url.push_str(&sentence.trim());

        let response = reqwest::get(&url).await;

        let response = match response {
            Ok(resp) => {
                if !resp.status().is_success() {
                    eprintln!("Request failed with code: {}", resp.status());
                    return Err(());
                }
                resp
            }
            Err(e) => {
                eprintln!("Failed to make request: {}", e);
                return Err(());
            }
        };
        
        let translation = response.json::<API_Response>().await;

        let translation = match translation {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Failed to parse JSON response: {}", e);
                return Err(());
            }
        };

        println!("\nTranslated: {}", translation.contents.translated);
    }
    Ok(())
}
