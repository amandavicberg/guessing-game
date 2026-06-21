use std::io;
use rand::{Rng, RngExt};
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("*Jogo de adivinhar o número*");
        println!();
        println!("Digite o seu palpite:");

        let mut guess= String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Falha ao ler entrada");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!();
        println!("Você disse: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Seu palpite foi muito baixo"),
            Ordering::Greater => println!("Seu palpite foi muito alto"),
            Ordering::Equal => {
                println!("Parabéns. Você acertou!");
                break;
            }
        };
        println!();
    }
}
