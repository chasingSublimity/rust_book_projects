/*
    Ownership Rules
    * Each value in rust has a variable that's called its owner
    * There can only be one owner at a time
    * When the owner goes out of scope, the value will be dropped
*/

fn main() {

    {    
        {
            // s is not valid here, not declared
            let string1 = "Hello"; // s is valid from this point 
            println!("{}", string1); // do stuff with s
        }
        // s is no longer valid

        // unlike string literals, "Strings" are stored on the heap.
        // This makes them mutable
        let mut string2 = String::from("hello");
        string2.push_str(", world!");

        println!("{}", string2);

        /*
            in order to support a mutable/growable string, 
            we must allocate an amount of memory on the heap.
            Because the amount isn't knowable at compile time:
                * the memory must be requested from the OS at runtime
                * we need a way to return the memory to the OS when we're done with the String
        */

        {
            let string3 = String::from("hello3"); // s enters scope
            println!("{}", string3);
        }
    }
    /* 
        s exits scope. rustc calls "drop"

        Resource Acquisition Is Initialization (RAII):
        the pattern of deallocating resources at 
        the end of an item's lifetime
    */

    {
        let _x = 5; // bind the value 5 to x
        let _y = _x; // then make a copy of the value in x and bind it to y

        /* 
            String is made of three parts:
                1) pointer to contents of string on heap
                2) len -- bytes of memory the string is currently using
                3) capacity -- bytes of memory the string has recieved from the OS

            these three parts are stored on the stack

        */
            
        let s1 = String::from("hell0");
        // String data (pointer, len, capacity) is copied on the stack,
        // The data on the heap that the pointer refers to is not.
        let _s2 = s1;

        // println!("{}", s1); <- This errors
        /*
            Because rust invalidates s1 on after being assigned to s2,
            a "move" occurs, not a shallow copy.

            Related: Rust will never automatically create 
            deep copies of your data. Therefore, any automatic
            copying can be assumed to be inexpensive in terms 
            of runtime performance

        */
    }

    // use clone for deep copies
    {
        let s1 = String::from("hello");
        let s2 = s1.clone();

        println!("s1: {}, s2: {}", s1, s2);
    }

    // ownership and functions
    {
        let s = String::from("hello");  // s comes into scope

        takes_ownership(s); // s's value moves into the function...
                            // ... and is no longer valid here

        let x = 5; // x comes into scope
        makes_copy(x)   // x would move into the function,
                        // but i32 is a Copy, so it's ok to still
                        // use x afterward
    } // Here, x goes out of scope, then s (fifo). However, s's value was moved, so nothing special happens

    {
        let s1 = gives_ownership(); // moves it's return value into s1

        let s2 = String::from("hello");

        // s2 is moved into takes_and_gives_back,
        // which also moves its return value into s3
        let s3 = takes_and_gives_back(s2);
        println!("s3: {}", s3);
    }
}

    fn gives_ownership() -> String {
        let some_string = String::from("hello");

        // return and move out to the calling function
        some_string // note, no semi
    }

    fn takes_and_gives_back(a_string: String) -> String {
        a_string // returned to the calling function
    }

    fn takes_ownership(mut some_string: String) {
        some_string.push_str(", world!");
        println!("{}", some_string);
    } // some_string is out of scope, `drop` is called

    fn makes_copy(some_integer: i32) { // some_integer comes into scope
        println!("{}", some_integer);
    } // some_integer goes out of scope. Nothing special happens