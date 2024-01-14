// El operador de control de flujo match
// Rust tiene una construcción de flujo de control extremadamente poderosa llamada match que te permite comparar un valor
// contra una serie de patrones y luego ejecutar código basado en qué patrón coincide

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        // Coin::Penny => 1,
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {}
