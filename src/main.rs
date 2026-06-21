use std::io;
use rand::{Rng, RngExt};

fn main() {
    println!("Adivinhe o número :)");

    let secret_number = rand::rng().random_range(1..=100);

    println!("O número aleatório é: {}", secret_number);

    println!("Digite o seu palpite:");

    let mut guess= String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Falha ao ler entrada");

    println!("Você disse: {}", guess);
}
