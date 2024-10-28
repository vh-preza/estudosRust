use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number! 1-100!");

    let number = rand::thread_rng()
        .gen_range(1..=100);

    loop{
        println!("Input your answer....");

        let mut answer = String::new();

        io::stdin()
            .read_line(&mut answer) 
            .expect("failed to read line");
    
        let answer: u32 = match answer.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
            

        println!("your answer is : {}", answer);

        match answer.cmp(&number){
            Ordering::Less => println!("the secret number is Higher..."),
            Ordering::Greater => println!("the secret number is Lower..."),
            Ordering::Equal => {
                println!("YOU WIN!!!!");
                break;
            }
        }
    }
}
