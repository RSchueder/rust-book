
pub fn new_vector() {

    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(4);
    println!("{:?}", v);

    let v = vec![1, 2, 3];
    println!("{:?}", v);

    let third: &i32 = &v[2];
    let opt_third: Option<&i32> = v.get(2);

    match opt_third{
        Some(opt_third) => println!("The third element is {opt_third}"),
        None=> println!("There is no third element")

    }

    // this would not work
    // v is mutable
    let mut v = vec![1, 2, 3, 4, 5];

    // first references memory address using immutable borrow
    let first = &v[0]; // declaration of the immutable borrow
    // the following would be a second mutable borrow that occurs on the variable while borrow is already out, and so it would fail
    //v.push(6);
    // this println involves an immutable borrow after a mutable borrow (v.push(6);)
    // you cannot use an immutable borrow after a mutable borrow, because this would
    // potentially invalidate the immutable borrow (let first = &v[0];) if the mutation
    // were to modify the memory that the immutable borrow relies on

    println!("The first element is: {first}");  // use of the immutable borrow

    // the following exmple shows us how mutable references work
    // see https://www.reddit.com/r/rust/comments/vvowkl/what_is_the_difference_between_mutable_variable/
    let first = &mut v[0];
    *first = *first + 1;
    let value = v.get(0);

    match value{
        Some(value) => println!("The first element is: {value}"),
        None=> println!("There is no first element")

    }

    for vv in &v{
        println!("{vv}");
    }

    let mut bigv = vec![100,200,300,400,500];

    for vv in &mut bigv{
        *vv = *vv + 75;
        println!("{vv}");


    }

}
