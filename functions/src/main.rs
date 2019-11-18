fn main() {
    another_function(45, 63);

// statements and expressions
    // statements are instructions that perform some action and
    // do not return a value. (therefore, function definitions are a statement)
    let _y = 6;
    
    // The below errors, because there is no return value for x to bind to
    // let _x = (let y = 6);

    // expressions evaluate and return something

    let z = {
        let q = 3;
        q + 1
    };
    println!("z is {}", z);

    let fiver = five();
    println!("fiver: {}", fiver);
}   

// in function singatures, type must be declared for each parameter.
// This is a deliberate design decision: using annotations here
// means the compiler almost never needs us to use them elsewhere
fn another_function(x: i32, y: i32) {
    println!("The vale of x is: {}", x);
    println!("The vale of y is: {}", y);
}

// most functions return the final expression implicitly
fn five() -> i32 {
    5
}