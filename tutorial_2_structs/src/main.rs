
// struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

// tuple struct, where field names would be redundant
struct Color(i8, i8, i8);
struct Point(f32, f32, f32);

// unit like structs without any fields, no data
struct AlwaysEqual;


// add an outer attribute
#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32
}

// here is how you implement a method for a struct

impl Rectangle {
    fn area(&self) -> u32 {
        // imutable borrow
        // if it were not a borrow, I suppose self would be deallocated after execution of the method?
        self.width * self.height
    }
    fn area_dealoc(self) -> u32 {
        // mutable borrow
        // if it were not a borrow, the method would take ownership of self and self would be deallocated after execution of the method
        // basically, if a variable enters a function, and it is not returned back by the function, then it dies in the function
        self.width * self.height
    }

    fn can_hold(&self, rect2: &Rectangle) -> bool {
        self.width > rect2.width && self.height > rect2.height
    }

    // this is an associated function that is not a method because it does not use self
    fn square(dimension: u32) -> Rectangle{
        Rectangle{
            width: dimension,
            height: dimension
        }
    }
}

fn main() {

    // we do not use references as we want the instance to own all
    // of its data
    let mut mike = User {
        active: true,
        email: String::from("rudy@gmail.com"),
        sign_in_count: 0,
        username: String::from("navier")
    };

    println!("{}", mike.username);
    println!("{}", mike.sign_in_count);
    println!("{}", mike.active);
    println!("{}", mike.email);

    mike.username = String::from("mike");
    println!("{}", mike.username);

    let new_user = build_user(String::from("mike@email.com"), String::from("mike2"));
    println!("{}", new_user.username);

    // make instance from another instance
    // here the data is moved from  1st user to second user
    let alias = User{
        username: String::from("rudy"),
        ..new_user
    };
    println!("{}", alias.username);

    // this would not work because email is a str, str is not on stack, and 
    // therefore strs do not implement Copy. Because it does not implement copy,
    // it gets moved on reassignment
    // println!("{}", new_user.email);

    let color = Color(5, 26, 78);
    let point = Point(1., 1., 3.);

    let scale = 2;
    // can debug a value using dbg! macro which returns ownership of output
    let rectangle = Rectangle{
        height: dbg!(2 * scale),
        width: 4
    };

    let rectangle2 = Rectangle{
        height: 2,
        width: 2
    };
    println!("{}", rectangle.can_hold(&rectangle2));

    // associated function as a constructor
    let square = Rectangle::square(6);
    
    // use debug output format provided by an outer attribute
    println!("{:?}", square);

    // memory is not deallocated inside dbg! macro because it was on the stack
    dbg!(scale);
    println!("{:?}", scale);

    let area = calculate_area(&rectangle);
    println!("{}", area);
    println!("{:?}", rectangle);
    let area = rectangle.area();
    println!("{}", area);

    // memory is dealocated/consumed inside dbg! macro. Turns out println! is just a special case where this does not happen
    // https://www.reddit.com/r/rust/comments/axgfjo/ownership_rules_when_calling_println_macro/
    // wouldn't happen for a variable on the stack  
    dbg!(rectangle);
    // this would fail
    // println!("{:?}", rectangle);

}

fn build_user(email: String, username: String) -> User {
    // uses field init where parameter names match field name
    User {
        email,
        username,
        active: true,
        sign_in_count: 2
    }
}

fn calculate_area(rectangle: &Rectangle) -> u32{
    // immutable borrow of the struct
    rectangle.height * rectangle.width
}