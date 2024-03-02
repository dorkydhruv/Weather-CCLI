use serde::Deserialize;
use reqwest;
#[derive(Deserialize,Debug)]
struct WeatherData{
    main: Main,
    weather: Vec<Weather>
}

#[derive(Deserialize,Debug)]
struct Main{
    temp: f64,
}

#[derive(Deserialize,Debug)]
struct Weather{
    description: String,
}

pub async fn get_weather(city:String, api_key:&str)->Result<(),reqwest::Error>{
    let url = format!("https://api.openweathermap.org/data/2.5/weather?q={}&appid={}",city.trim(),api_key,);
    let response = reqwest::get(&url).await?;
    //Debugging the response status
    print!("The response is -> {:?}",response.status());
    if response.status().is_success(){
        let weather_data:WeatherData = response.json().await?;
        let temp = weather_data.main.temp;
        let description = &weather_data.weather[0].description;
        println!("The temperature in {} is {:.2} C, {}.",city.trim(),temp-273.15,description);
    }else{
        println!("Failed to get weather data for {}.",city.trim());
    }
    Ok(())
}