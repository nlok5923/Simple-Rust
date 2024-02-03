use reqwest;
use std::fs::File;
use std::io::Write;

#[derive(Debug)]
struct SampleStruct {
    data: String,
}

#[derive(Debug)]
struct ApiResponse {
    status_code: reqwest::StatusCode,
    body: String,
}

async fn fetch_data_and_process(
    sample_struct: &SampleStruct,
) -> Result<ApiResponse, reqwest::Error> {
    let url = "https://jsonplaceholder.typicode.com/posts/1";

    let client = reqwest::Client::new();

    let response = client.get(url).send().await?;

    let body = response.text().await?;

    let processed_data = format!("Processed: {} - {}", sample_struct.data, body);

    let mut data_file = File::create("data.txt").expect("creation failed");

    data_file.write(processed_data.as_bytes()).expect("write failed");

    Ok(ApiResponse {
        status_code: reqwest::StatusCode::OK,
        body: processed_data,
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let sample_struct = SampleStruct {
        data: "Hello, Rust!".to_string(),
    };

    match fetch_data_and_process(&sample_struct).await {
        Ok(response) => {
            println!("Response: {:?}", response);
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
        }
    }

    Ok(())
}
