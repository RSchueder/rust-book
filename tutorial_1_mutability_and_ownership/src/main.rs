fn main() {
    // memory allocation of ints is known at compile time and is allocated on the heap rather than the stack
    // because x is on the stack, y copies the data
    let x = 5;
    let y = x;
    println!("x is {}", x);

    println!("y is {}", y);

    // because str1 is not on the stack, only the pointer is copied, and not the data
    let str1 = String::from("hello");
    let str2 = str1; // str1 is considered no longer valid, and memory does not need to be freed
    // i.e., str1 was moved to str2. Any automatic copying can be assumed to be inexpensive
    println!("str2 is {}", str2);
    // to make a copy we can clone all of the data
    let str3 = str2.clone();
    println!("str3 is {}", str3);

    // when variable on heap is sent to function, current scope loses ownership
    let str4 = String::from("This is a string");
    takes_ownership(str4);

    let x = 4;
    makes_copy(x);
    println!("{}", x);

    let str4 = String::from("This is a string");
    let mut str4 = takes_and_gives_back(str4);
    println!("{}", str4);

    // pass reference to function to not pass on ownership
    let length = calculate_length(&str4);
    println!("{}", length);
    augment(&mut str4);

    // cannot have >1 mutable references
    // let r1 = &mut s
    // let r2 = &mut s

    // allowed sequentially in seperate scope
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    }
    
    let r2 = &mut s;

    // cannot have mutable and immutable references
    // let r1 = &s
    // let r2 = &s
    // let r3 = &mut s
    // fine if those variables go out of scope

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point
    let r3 = &mut s; // no problem
    println!("{}", r3);

    let long_str = String::from("This is a long sentence.");
    let first = first_word(&long_str);
    println!("{}", first);

    // string slices referring to specific ranges
    let beginning = &long_str[0..5];
    let end = &long_str[6..11];
    println!("{}", beginning);
    println!("{}", end);

    let binary_str = "Ooops, this is not a String"; // this is technically a slice
    let first = first_word(&binary_str);
    println!("{}", first);
}

fn takes_ownership(some_string: String){
    // function scope now owns variable
    println!("{}", some_string);
    // memory is deallocated as it leaves this scope without transfer of ownership
}

fn makes_copy(some_integer: i32){
    println!("{}", some_integer);
    // memory is not dealloctaed because data was on stack
}

fn takes_and_gives_back(some_string: String) -> String {
    some_string // is returned and ownership is transferred
}

fn calculate_length(str: &String) -> usize {
    // does not take ownership of variables entering this scope, only looks at data at pointer
    str.len()
}

fn augment(str: &mut String)  {
    // does not take ownership of variables entering this scope, only looks at data at pointer
    // we can allow the input to be a mutable reference
    str.push_str(", please");
}

fn first_word(s: &str) -> &str {
    // return the the first word in the string
    // Defining a function to take a string slice instead of a reference to a String 
    // makes our API more general and useful without losing any functionality
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // returns a single value that ties back to the actuall underlying data
            return &s[..i];
        }
    }

    &s[..]
}