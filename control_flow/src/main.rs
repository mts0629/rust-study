fn main() {
    let number = 3;

    // if expressions, sometime called `arms`
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // The condition must be a bool
    // Compliation error: mismatched types
    // if number {
    //     println!("number was three");
    // }

    if number != 0 {
        println!("number was something other than zero")
    }

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        // The first body which the condition evaluated to true is executed
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    // Using if in a let statement
    let number = if condition { 5 } else { 3 };

    println!("The value of number is: {number}");

    // Compliation error: incompatible types
    // let number = if condition { 5 } else { "six" };
}
