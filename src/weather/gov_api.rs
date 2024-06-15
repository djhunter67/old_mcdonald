enum WeatherError {
    BadReply,
}

impl From<reqwest::Error> for WeatherError {
    fn from(_: reqwest::Error) -> Self {
        WeatherError::BadReply
    }
}

/// Get the weather forcast based on the latitude and longitude
async fn get_weather(long: &str, lat: &str) -> Result<String, WeatherError> {
    let lat = "39.7456";
    let long = "-97.0892";
    let request_params = reqwest::get(format!("https://api.weather.gov/points/{lat},{long}"))
        .await?
        .text()
        .await?;

    Ok(request_params)
}
