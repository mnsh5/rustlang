use std::io; // biblioteca estandar de entrada/salida io
             // std::io te proporciona una serie de características útiles, incluyendo la capacidad de aceptar la entrada del usuario

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
