fn main() {
    println!("attempt to add with overflow");

    let s: i32 = 5;
    println!("{}", s);


    let mut i: i128 = 1;
    let mut vec = Vec::new();
    loop {
        {
        vec.push(i);
        }
        i += i;
        println!("{}", i);
    }
    
}