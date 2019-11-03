fn main() {
// scalar types
    // integers
    let x: i8 = 45; // signed 8 bit integer
    println!("{}", x); // 45

    // floats
    let y = 2.0; // f64, double precision
    let z: f32 = 3.0; // f64, single precision
    println!("f64 {}, f32 {}", y, z); // f64 2, f32 3

    // numeric operations
    let _sum = 5 + 10;
    let _difference = 95.5 - 4.3;
    let _product = 4 * 30;
    let _quotient = 56.7 / 32.2;
    let _remainder = 43 % 5;

    // booleans
    let _t = true;
    let _f: bool = false;

    // chars
    let _c = 'z';
    let _z = 'Z';
    let _heart_eyed_cat = '😻';

// compound types

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_1, _2, _3) = tup;
    println!("the value of _2 is: {}", _2);
    println!("{}", tup.0);

    // array
    let a = [1,2,3,4,5]; // every element must be same type

}
