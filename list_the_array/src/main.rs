fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
    
    println!("");   // leaves a blank space between both loops

    // with a for loop
    // a is defined already
    for element in a {  // any word or letter can be used instead of the word 'element' - for i in a, for x in a; etcetera
        println!("the value is: {element}");
    }

    println!("\n");     // prints 2 blank lines

    // with a for loop using range values
    // for a number in 1 through 4 - reversed...
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}