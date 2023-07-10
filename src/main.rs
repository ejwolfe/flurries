use flurries::{get_forecast, get_weather, setup_environment};

fn main() {
    let service = setup_environment();
    let weather = get_weather(&service);
    let description = if weather.weather.len() > 0 {
        &weather.weather[0].description
    } else {
        ""
    };
    println!("{}", &weather.name);
    println!("{}°", &weather.main.temp.round());
    println!("{}", description);
    println!(
        "H:{}° L:{}°",
        &weather.main.temp_max.round(),
        &weather.main.temp_min.round()
    );
    let forecast = get_forecast(&service);
    println!("");
    println!("{}", &forecast.city.name);
    println!("{}", &forecast.list[0].pop);
}

enum Conditions {
    ClearSky,
    FewClouds,
    ScatteredClouds,
    BrokenClouds,
    RainShower,
    Rain,
    Thunderstorm,
    Snow,
    Mist,
}

impl Conditions {
    fn as_day_str(&self) -> &'static str {
        match self {
            Conditions::ClearSky => "☀️",
            Conditions::FewClouds => "⛅",
            Conditions::ScatteredClouds => "☁️",
            Conditions::BrokenClouds => "☁️",
            Conditions::RainShower => "🌧️",
            Conditions::Rain => "☔",
            Conditions::Thunderstorm => "🌩️",
            Conditions::Snow => "🌨️",
            Conditions::Mist => "🌫️",
            _ => "",
        }
    }
    fn as_night_str(&self) -> &'static str {
        match self {
            Conditions::ClearSky => "🌑",
            Conditions::FewClouds => "☁️",
            Conditions::ScatteredClouds => "☁️",
            Conditions::BrokenClouds => "☁️",
            Conditions::RainShower => "🌧️",
            Conditions::Rain => "☔",
            Conditions::Thunderstorm => "🌩️",
            Conditions::Snow => "🌨️",
            Conditions::Mist => "🌫️",
            _ => "",
        }
    }
}

fn print_weather_condition(code: i64) -> String {
    let test = Conditions::ClearSky.as_day_str();
    match code {
        200 => String::from("⛈️"),
        201 => String::from("⛈️"),
        202 => String::from("⛈️"),
        210 => String::from("⛈️"),
        211 => String::from("⛈️"),
        212 => String::from("⛈️"),
        221 => String::from("⛈️"),
        230 => String::from("⛈️"),
        231 => String::from("⛈️"),
        232 => String::from("⛈️"),
        _ => String::from(""),
    }
}
