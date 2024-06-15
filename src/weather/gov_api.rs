use actix_web::HttpRequest;

enum WeatherError {
    BadRequest,
    BadReply,
    BadWeather,
}

/// Get the weather forcast based on the latitude and longitude
fn get_weather(long: &str, lat: &str) -> Result<String, WeatherError> {
    // let request_params =

    Ok(String::new())
}
