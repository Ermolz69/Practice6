use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize, Serializer, Deserializer};
use serde_json::{self};
use std::fs::File;
use std::io::Read;
use std::time::Duration;
use url::Url;
use uuid::Uuid;
use serde_yaml::to_string as to_yaml;
use toml::to_string as to_toml;

#[derive(Debug, Deserialize, Serialize)]
struct PublicTariff{
    id: u32,
    price: u32,
    #[serde(with = "humantime_serde")]
    duration: Duration,
    description: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct PrivateTariff{
    client_price: u32,
    #[serde(with = "humantime_serde")]
    duration: Duration,
    description: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Stream{
    user_id: uuid::Uuid,
    is_private: bool,
    settings: u32,
    shard_url: Url,
    public_tariff: PublicTariff,
    private_tariff: PrivateTariff,
}

#[derive(Debug, Deserialize, Serialize)]
struct Gift{
    id: u32,
    price: u32,
    description: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Debug{
    #[serde(with = "humantime_serde")]
    duration: Duration,
    at: chrono::DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Request{
    #[serde(rename = "type")]
    request_type: RequestType,
    stream: Stream,
    gifts: Vec<Gift>,
    debug: Debug,

}

#[derive(Debug, Deserialize, Serialize)]
enum RequestType {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failure")]
    Failure
}

fn main(){
    let event = Event {
        name: "Event 1".to_string(),
        date: "2024-11-14".to_string(),
    };

    let json = serde_json::to_string(&event).unwrap();
    print!("\n\n {}", json);

    let des_event: Event = serde_json::from_str(&json).unwrap();
    println!("{:?}", des_event);

    
    /*
    let mut file = File::open("request.json").unwrap();
    let mut json_str = String::new();
    file.read_to_string(&mut  json_str).unwrap();

    //print!("{}", json_str);

    let request: Request = serde_json::from_str(&json_str).unwrap();
    //print!("{:?}", request);
    let yaml_str = to_yaml(&request).unwrap();
    println!("YAML:\n{}", yaml_str);

    let toml_str = to_toml(&request).unwrap();
    println!("TOML:\n{}", toml_str);
    */
}

#[derive(Debug, Serialize, Deserialize)]
struct Event {
    name: String,
    #[serde(
        serialize_with = "serialize_date",
        deserialize_with = "deserialize_date"
    )]
    date: String,
}
 
fn serialize_date<S: Serializer>(date: &str, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&format!("Date: {}", date))
}
 
fn deserialize_date<'de, D: Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
    let data: &str = Deserialize::deserialize(deserializer)?;
    Ok(data.replace("Date: ", ""))
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_1(){
        let mut file = File::open("request.json").unwrap();
        let mut json_str = String::new();
        file.read_to_string(&mut json_str).unwrap();

        let request: Request = serde_json::from_str(&json_str).unwrap();

        assert_eq!(request.stream.public_tariff.price, 100);

        let expected_user_id = Uuid::parse_str("8d234120-0bda-49b2-b7e0-fbd3912f6cbf").unwrap();
        assert_eq!(request.stream.user_id, expected_user_id);

        assert_eq!(request.stream.public_tariff.duration,
        Duration::from_secs(3600)
        );

        assert_eq!( request.gifts[0].id, 1);

        assert_eq!(request.gifts.len(), 2);


    }
}

/* 
fn main() {
    let user = User {
        name:String::from("Iван"),
        email:String::from("Ivan@gmail.com"),
        birthdate:String::from("1991-01-01"),
    };

    let json = serde_json::to_string(&user).expect("Помилка of Serialize");
    print!("{}", json);

    let deserialized_user: User = serde_json::from_str(&json).expect("Помилка of Deserialize");
    print!("{:?}", deserialized_user);
}
*/