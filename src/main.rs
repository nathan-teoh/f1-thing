
use f1_stuff::models::Seasons;
use reqwest::Error;
use serde::{Serialize,Deserialize};
use serde_json::json;


#[tokio::main]
async fn main() -> Result<(),Error>{
    let REQUEST_URL: String = format!("http://ergast.com/api/f1/seasons.json?limit={limits}",
                                            limits="100");
    let response: reqwest::Response = reqwest::get(&REQUEST_URL).await?;
    let seasonss:serde_json::Value = response.json().await?;
    let parsed: serde_json::Value = seasonss["MRData"]["SeasonTable"].clone();

    println!("{}",parsed);
    
    // let zz: Vec<Seasons> = serde_json::from_value(parsed).unwrap();
    // for z in zz{
    //     println!("{:?}",z);
    // }
    Ok(())
}
