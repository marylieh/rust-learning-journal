fn main() {
    let celsius: f32 = 19.0;
    let fahrenheit: f32 = convert(celsius);

    println!("{}, Celsius is {}, Fahrenheit", celsius, fahrenheit);
}

fn convert(celsius: f32) -> f32 {
    let fahrenheit = (celsius * 1.8) + 32.0;
    return fahrenheit;
}
