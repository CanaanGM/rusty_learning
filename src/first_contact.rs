
// the plan is to contact a server to get some info 

#[tokio::main]
async fn first_contact() {
    
  let greeting = greet_fast_alien().await;  
    println!("Greetings = {:#?}", greeting);
}

pub async fn greet_fast_alien() -> Result<String, Box<dyn std::error::Error>>{
    let client = reqwest::Client::new();
    let body = client.get("http://localhost:8989")
        .send()
        .await?
        .text()
        .await?;
    println!("{}", body);
    //{"Greetings ~! ":"from your lovely neighbor fast alien ~!"}
    // deserialize maybe ? 
    Ok(body)
}
