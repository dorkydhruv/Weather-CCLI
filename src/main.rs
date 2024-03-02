mod lib;
#[tokio::main]
async fn main(){
    let api_key = "YOUR_API_KEY";
    let mut city = String::new();
    println!("Enter the city name -> ");
    std::io::stdin().read_line(&mut city).expect("Failed to read input");
    let city_clone = city.clone();
    tokio::spawn(lib::get_weather(city_clone, api_key));
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
}
