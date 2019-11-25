fn main() {
    let number = 7;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    if number != 0 {
        println!("number was not zero");
    }

    let condition = true;
    // if statements are an expression, 
    // so they can be used on the right side of a let statement
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is : {}", number)
}
