

fn is_even(n : i32) -> bool {
    if n % 2 == 0 {
        return true;
    }
    return false;
}

fn fizzbuzz (n: i32) -> String {
    if n % 15 == 0 {
        return "FizzBuzz".to_string();
    } 
    else if n % 3 == 0 {
        return "Fizz".to_string();
    }
    else if n % 5 == 0 {
        return "Buzz".to_string();
    }
    return "Not divisible by 5, 3, or 15".to_string();
}

fn find_sum_of_array (array: &[i32]) -> i32 {
    let mut sum = 0;
    for i in 0..array.len() {
        sum += array[i];
    }
    return sum;
}

fn find_largest_num_in_array (array: &[i32]) -> i32 {
    let mut largest_num = 0;
    for i in 0..array.len() {
        if array [i] >= 0 {
            largest_num = array[i];
        }
    }
    return largest_num;
}

fn main() {
    let mut my_array = [1,2,3,4,5,6,7,8,9,10];
    println!("My ten integers: {:?}", my_array);

    for i in 0..my_array.len() {
        let array_val = my_array[i];
        println! ("Analyzing Number ---- {}", array_val);
        
        println!("Is {} even?: {}", array_val, is_even(array_val));
        println!("Divisble?: {}", fizzbuzz(array_val));

        println!("------------------");
    }
    println!("********************");
    println!("Sum of Array: {}", find_sum_of_array(&my_array));
    println!("Largest Number in Array: {}", find_largest_num_in_array(&my_array));
    println!("********************");
}
