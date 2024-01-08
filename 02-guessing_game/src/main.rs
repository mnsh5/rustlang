use rand::Rng;
use std::cmp::Ordering;
// biblioteca estandar de entrada/salida io
// std::io te proporciona una serie de características útiles, incluyendo la capacidad de aceptar la entrada del usuario
use std::io;

fn main() {
    println!("Guess the number!");

    // función rand::thread_rng que nos da el generador de números aleatorios particular
    // El método gen_range toma una expresión de rango como argumento y genera un número aleatorio en el rango
    // 1..=100 para solicitar un número entre 1 y 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    //  loop crea un bucle infinito
    loop {
        println!("Please input your guess.");

        // declaración let para crear la variable
        // variable mutable guess
        // String::new() función que devuelve una nueva instancia de un String
        // La sintaxis :: en la línea ::new indica que new es una función asociada del tipo String
        let mut guess = String::new(); // ha creado una variable mutable que está actualmente enlazada a una nueva instancia vacía de un String

        // función stdin del módulo io, que nos permitirá manejar la entrada del usuario
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {guess}");

        // Comparando la adivinanza con el Número Secreto

        // Shadowing de guess
        // parse devuelve un tipo Result y Result es un enum que tiene las variantes Ok y Err
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // El tipo Ordering es otro enum y tiene las variantes Less, Greater y Equal. Estos son los tres resultados posibles cuando compara dos valores
        // El método cmp compara dos valores y se puede llamar en cualquier cosa que se pueda comparar
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
