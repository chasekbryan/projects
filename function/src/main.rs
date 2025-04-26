fn five() -> i32 {
    5
}   // this is a function

fn main() {
    let x = five();     // this references the function and assigns it's value to x

    println!("The value of x is: {x}");
}