
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
    // IpAddr::V4() is a function call that takes a String argument and returns an instance of the IpAddr type. 
    // We automatically get this constructor function defined as a result of defining the enum.
    let ip4 = IpAddrKind::V4(127,0,0,1);
    let ip6 = IpAddrKind::V6(String::from("::1"));

}
