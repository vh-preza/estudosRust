
enum Choice{
    Num(i32),
    Word(String),
}

fn main() {
    
    let z = Choice::Num(8);

    let v = Choice::Word(String::from("Hello_world!"));

    match z{
        
        Choice::Num(num) => println!("{num}"),

        Choice::Word(s) => println!("{s}"),
    }

    match v{
        
        Choice::Num(num) => println!("{num}"),

        Choice::Word(s) => println!("{s}"),
    }
}
