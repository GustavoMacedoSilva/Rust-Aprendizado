use std::cmp::Ordering;
use std::io;
use rand::Rng;


fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop{
        println!("Please input your guess. ");

        let mut guess = String::new();


        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Input a real number, dumbass!");
                continue;
            }
        }; // pega o valor
    // da String guess, com o trim corta os whitespaces e com o parse tranformamos no tipo que
    // queremos no 'guess: u32'
    // u32 eh bom para o exemplo, ele eh um unsigned integer number de 32 bits
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big! OwO"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
