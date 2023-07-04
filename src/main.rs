fn main() {
    println!("Hello, world!");
    request_data();
}

fn request_data() {
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
