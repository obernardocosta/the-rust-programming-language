fn c31() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("THREE_HOURS_IN_SECONDS: {}", THREE_HOURS_IN_SECONDS);


    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of x in the inner scope is: {}", y);
    }

    println!("The value of x is: {}", y);
}

fn c32(){
    
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;
 
    // multiplication
    let product = 4 * 30;
 
    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0
 
    // remainder
    let remainder = 43 % 5;

    println!("{} {} {} {} {} {}", sum, difference, product, quotient, floored, remainder);

    let t = true;

    let f: bool = false; // with explicit type annotation

    println!("{} {}", t, f);

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("{} {} {}", c, z, heart_eyed_cat); 


    let tup1: (i32, f64, u8) = (500, 6.4, 1);
    let tup2 = (500, 6.4, 1);

    let (x, y, z) = tup1;

    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("{} {} {}", five_hundred, six_point_four, one);

    let a1 = [1, 2, 3, 4, 5];
    let a2: [i32; 5] = [1, 2, 3, 4, 5];
    let a3 = [3; 5];


    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];

    println!("{}", a1[0]);
    println!("{}", months[1]);
}

fn c331(x: i32){
    println!("{}", x);
}

fn c332(y: i32) -> i32{
    y + 1
}

fn c333(mut y: i32) -> i32{
    y = y + 2;
    y + 1
}

fn if_function(){
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }

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

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

}

fn loops() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
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
    println!("End count = {}", count);


    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);


    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

}

fn main() {
    c31();
    c32();
    let x = 4;
    c331(x);
    let y = c332(x);
    println!("{}", y);
    let y = c333(x);
    println!("{}", y);
    if_function();
    loops();

    /*
    Comments
    */

    // comment
}
