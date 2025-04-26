fn main() {
    let mut counter = 0;
    
    let result = loop {
        counter += 1;   // incremented variable

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {result}");    // prints 20
}