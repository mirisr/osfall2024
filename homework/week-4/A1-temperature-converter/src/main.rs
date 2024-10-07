
const FARENHEIT:f64 = 32.0;

fn fahrenheit_to_celsius(f:f64) -> f64 {
    return (f - FARENHEIT) * 5.0 / 9.0;
}

fn celsius_to_fahrenheit(f:f64) -> f64 {
    return FARENHEIT +  (9.0 / 5.0) * f;
}

fn main() {

    let mut temp_in_fahrenheit = 32.0;
    let x = fahrenheit_to_celsius(temp_in_fahrenheit);
    println!("The temperature in fahrenheit {} is {} in celsius", temp_in_fahrenheit, x);

    for i in 1..6 {
        let mut next_temp_in_fahrenheit = temp_in_fahrenheit + i as f64;
        println!("temperature in fahrenheit {} is {} in celsius", next_temp_in_fahrenheit, fahrenheit_to_celsius(next_temp_in_fahrenheit));
    }

    // let x = fahrenheit_to_celsius(104.0);
    // println!("{}", x);

    // let mut temp_array = [33.0, 34.0, 35.0, 36.0, 37.0];
    // let mut converted_temp_array = [0.0; 5];
    // for i in 0..5 {
    //     let mut temp = temp_array[i];
    //     let mut converted_temp = fahrenheit_to_celsius(temp);
    //     converted_temp_array[i] = converted_temp;

    //     let mut converted_back = celsius_to_fahrenheit(converted_temp_array[i]);
    //     println!("From F to C: {} {} and back to {}", temp, converted_temp, converted_back);
    // }
}
