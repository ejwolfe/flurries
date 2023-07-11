use colored::{Colorize, CustomColor};
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
    println!("{}°", print_color(weather.main.temp.round()));
    println!("{}", description);
    println!(
        "H:{}° L:{}°",
        print_color(weather.main.temp_max.round()),
        print_color(weather.main.temp_min.round())
    );
    let forecast = get_forecast(&service);
    println!("");
    println!("{}", &forecast.city.name);
    println!("{}", &forecast.list[0].pop);
}

fn print_color(temp: f64) -> colored::ColoredString {
    let temp_string = temp.clone().to_string();
    let output = match temp {
        x if x < -40.0 => temp_string.custom_color(CustomColor::new(112, 41, 99)),
        x if x < -30.0 => temp_string.custom_color(CustomColor::new(48, 25, 52)),
        x if x < -20.0 => temp_string.custom_color(CustomColor::new(128, 0, 128)),
        x if x < -10.0 => temp_string.custom_color(CustomColor::new(93, 63, 211)),
        x if x < 0.0 => temp_string.custom_color(CustomColor::new(100, 149, 237)),
        x if x < 10.0 => temp_string.custom_color(CustomColor::new(0, 71, 171)),
        x if x < 20.0 => temp_string.custom_color(CustomColor::new(0, 150, 255)),
        x if x < 30.0 => temp_string.custom_color(CustomColor::new(125, 249, 255)),
        x if x < 40.0 => temp_string.custom_color(CustomColor::new(8, 143, 143)),
        x if x < 50.0 => temp_string.custom_color(CustomColor::new(175, 225, 175)),
        x if x < 60.0 => temp_string.custom_color(CustomColor::new(80, 200, 120)),
        x if x < 70.0 => temp_string.custom_color(CustomColor::new(255, 234, 0)),
        x if x < 80.0 => temp_string.custom_color(CustomColor::new(255, 191, 0)),
        x if x < 90.0 => temp_string.custom_color(CustomColor::new(233, 116, 81)),
        x if x < 100.0 => temp_string.custom_color(CustomColor::new(238, 75, 43)),
        x if x >= 100.0 => temp_string.custom_color(CustomColor::new(136, 8, 8)),
        _ => "".normal(),
    };
    output
}
