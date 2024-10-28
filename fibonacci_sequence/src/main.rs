use std::io;

fn main() {
    println!("insert the nth number in the fibonnaci sequence:");

    let mut number = String::new();
    let result : u32 = 0;

    io::stdin()
        .read_line(&mut number)
        .expect("failed to read line");
    
    let number : u32 = number.trim()
        .parse()
        .expect("data inserted whas not a number");

    if number == 0 { 
        println!("{}", result);
    }
    else{
        let result : u32 = fibonnaci(number);
        println!("{}", result);
    }
}

fn fibonnaci(number: u32 ) -> u32 {
    if number == 1
    {
        0
    }
    else if number == 2
    {
        1
    }
    
    fibonnaci(number - 1) + fibonnaci(number - 2)
}