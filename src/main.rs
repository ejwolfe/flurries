use std::{
    env::{set_var, var},
    fs,
    str::Split,
};

fn main() {
    println!("Hello, world!");
    read_env_file();
    request_data();
}

fn request_data() {
    let api_key = match var("API_KEY") {
        Ok(api_key) => api_key,
        Err(_) => String::from(""),
    };
    println!("{}", api_key);
    let response = reqwest::blocking::get("https://jsonplaceholder.typicode.com/todos/1");
    let result = match response {
        Ok(result) => result,
        Err(error) => panic!("Error retreiving data: {}", error),
    };
    let data = match result.text() {
        Ok(text) => text,
        Err(error) => panic!("Error parsing text: {}", error),
    };
    println!("Data: {}", data);
}

fn read_env_file() {
    let file_path = ".env";
    let file = fs::read_to_string(file_path);
    if let Ok(contents) = file {
        println!("{}", contents);
        parse_env_file(contents);
    } else {
        panic!("Environment file does not exist!");
    }
}

fn parse_env_file(data: String) {
    let lines = data.lines();
    for line in lines {
        let mut value = line.split("=");
        let key = get_next_value(&mut value);
        let value = get_next_value(&mut value);
        set_var(key, value);
    }
}

fn get_next_value(split: &mut Split<&str>) -> String {
    if let Some(value) = split.next() {
        value.to_string()
    } else {
        panic!("There is no next value")
    }
}
