fn main() {
    // memory allocation of ints is known at compile time and is allocated on the stack rather than the heap
    // because x is on the stack, y copies the data since stack copies are cheap
    let x = 5;
    let y = x;
    println!("x is {}", x);
    println!("y is {}", y);

    // because str1 is not on the stack, only the pointer, length and capacity are copied (on the stack), 
    // and not the data, which resides on the heap
    let str1 = String::from("hello");
    
    // str1 and str2 cannot point to the same memory, since when they went out of scope there would be a double free error
    // therefore, here the variable str1 is considered no longer valid, but does not have to be dropped (since it doesn't own anything I guess)
    let str2 = str1; 
    // i.e., str1 was moved to str2. Any automatic copying can be assumed to be inexpensive since it will never be a deeopcopy
    println!("str2 is {}", str2);
    // to make a copy we can clone all of the data, from heap to heap
    let str3 = str2.clone();
    println!("str3 is {}", str3);

    // must consider if the Data Type implements a Copy trait. This is how you know if it
    // gets copied on the stack or not.

    // when variable on heap is sent to function, current scope loses ownership
    // Passing a variable to a function will move or copy, just as assignment does
    // a string is on heap, so it is moved, not copied, when passed to this function, meaning
    // it leaves this scope
    // When a variable that includes data on the heap goes out of scope, 
    // the value will be cleaned up by drop unless ownership of the data has been moved to another variable.
    let str4 = String::from("This is a string");
    takes_ownership(str4);
    // can no longer use str4 since it was moved

    // an integer is on stack, meaning it is copied when passed to this function
    let x = 4;
    makes_copy(x);
    // can still use x since it was copied
    println!("{}", x);

    let str4: String = String::from("This is a string");
    let str5 = takes_and_gives_back(str4);
    println!("{}", str5);
    let str5 = takes_and_gives_back(str5);
    println!("{}", str5);

    let mut str6 = takes_and_gives_back(str5);
    // references allow you to pass variables to a function without taking ownership
    let length = calculate_length(&str6);
    println!("length of {} is {}", str6, length);
    // pass a mutable reference to mutate without taking ownership
    println!("Before {}", str6);
    augment(&mut str6);
    println!("After {}", str6);

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
    // println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point, so this is okay
    // it would not be okay to use r1, r2, and r3 simultaneously  
    let r3 = &mut s; // no problem
    println!("{}", r3);

    let long_str = String::from("This is a long sentence.");
    let first = find_first_word(&long_str);
    println!("{}", first);

    // string slices referring to specific ranges
    let beginning = &long_str[0..5];
    let end = &long_str[6..11];
    println!("{}", beginning);
    println!("{}", end);

    let binary_str = "Ooops, this is not a String"; // this is technically a slice
    let first = find_first_word(&binary_str);
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

fn find_first_word(s: &str) -> &str { // &str means it is a string slice
    // return the the first word in the string
    // Defining a function to take a string slice instead of a reference to a String 
    // makes our API more general and useful without losing any functionality
    let bytes = s.as_bytes();

    // Because we get a reference to the element from .iter().enumerate(), we use & in the pattern.
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // returns a single value that ties back to the actual underlying data
            return &s[..i]; // I think here & is just part of how you write string slice, since a string slice is a reference
        }
    }

    return &s[..]
}