fn main() {
    _if();
    _if_else();
    _if_in_let();
    _repetition_with_loops();
    _while_loops();
    _for_loops();
}

fn _for_loops() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

//  while
    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

//  For in
    for element in a {
        println!("the value is: {element}");
    }

//  Range
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
fn _while_loops() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn _repetition_with_loops(){
    // Infinite loop!
    // loop {
    //     println!("again!");
    // }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
    println!("==============================");


    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
    println!("==============================");
}

fn _if_in_let() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // type mismatch
    // let number = if condition { 5 } else { "six" };
    // println!("The value of number is: {number}");
}

fn _if_else() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn _if() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // err: mismatched types: not bool
    // if number {
    //     println!("number exists");
    // }

    if number != 0 {
        println!("number was something other than zero");
    }
}