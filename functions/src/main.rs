fn main() {
    let x = plus_one(5);

    let y = divided_by_twelve(4.0);

    println!("The value of x is: {x}");

    println!("The value of 4 divided by twelve is: {y}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn divided_by_twelve(y: f64) -> f64 {
    y / 12.0
}