// Option<T> has two variants, Some(i) and None
// When using Option<T> you must have code to deal with each variant
// You cannot do operations on a T (type) and an Option<T>, because Option<T> could be Some or None
//  https://doc.rust-lang.org/std/option/enum.Option.html

#[derive(Debug)] // so we can inspect the state in a minute
enum USState {
    Alabama,
    Alaska
}

enum Coin {
    Penny, 
    Nickel,
    Dime,
    Quarter(USState),
    Dollar
}


fn value_in_cents(coin: Coin) -> Option<u8> {
    match coin {
        // pattern match arm: pattern and expression
        // matches are exhaustive, meaning if there is no arm for each
        // variant of the enum an error will be raise.
        Coin::Penny => return Some(1),
        Coin::Nickel => return Some(5),
        Coin::Dime => return Some(10),
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
                        return Some(25)
        },
        // we can use `other`, or even `_` for the catch-all case
        // if i wanted to do nothing for a case I could use the unit expression `()` in the arm
        _other => {
            println!("100 or more");
            // this is testing the usage and handling of the Option enum,
            // Which permits the functionality of a Null but ensures the Nulls
            // cannot be used as their non-null counterparts
            return None
    } }
}


// if let
// you can think of if let as syntax sugar for a match that runs code when 
// the value matches one pattern and then ignores all other values.

fn plus_one(x: Option<i32>) -> Option<i32> {
    // match against an enum, bind a variable to the data inside, 
    // and then execute code based on it. It’s a bit tricky at first, 
    // but once you get used to it, you’ll wish you had it in all languages.
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}
// enum
enum IpAddrKind{
    V4(u8, u8, u8, u8),
    V6(String)
}

// if we used the different structs, which each have their own type, 
// we couldn’t as easily define a function to take any of these kinds of messages as we could with the Message enum defined in Listing 6-2, which is a single type.

enum Message{
    Move {x: u32, y: u32},
    Quit,
    Write(String),
    ChangeColor(i8, i8, i8)
}

fn main() {
    // IpAddr::V6() is a function call that takes a String argument and returns an instance of the IpAddr type. 
    // We automatically get this constructor function defined as a result of defining the enum.
    let ip4 = IpAddrKind::V4(127,0,0,1);
    let ip6 = IpAddrKind::V6(String::from("::1"));
    let quarter = Coin::Quarter(USState::Alabama);
    let pennies = value_in_cents(quarter);
    if pennies.is_some() {
        println!("Number of cents = {}", pennies.unwrap_or_default())
    }

    let dollar = Coin::Dollar;
    let pennies = value_in_cents(dollar);
    if pennies.is_some() {
        println!("Number of cents = {}", pennies.unwrap_or_default())

    }   else {
            println!("Actually, number of cents  >= 100")
    }

    // if let
    // you can think of if let as syntax sugar for a match that runs code when 
    // the value matches one pattern and then ignores all other values.
    // Good to use when the exhaustiveness is too verbose

    let max_val = Some(38); // Calling this a Some is calling it an Option since Some is a variant of Option
    match max_val {
        // this is basically saying "is max val a Some or a None? Bind its value to max if it is some"
        // binding the value of max_val to max is essentially retrieving its value
        Some(max) => println!("The max val was {}", max),
        None => println!("The max val was None"),
    
    }
    // here is if let if you just want to do a single case and ignore all the rest
    if let Some(max) = max_val {
        println!("The max val was {}", max)
    } 
        else {
            println!("The max val was None")
        }


}
