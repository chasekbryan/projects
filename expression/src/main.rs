fn main() {
    let y = {
        let x = 3;
        x + 1       // << notice the lack of semicolon because it is an expression
    };

    println!("The value of y is: {y}");    // 4
}
