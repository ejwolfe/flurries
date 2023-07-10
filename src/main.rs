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
