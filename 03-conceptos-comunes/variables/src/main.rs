fn main() {
    // variable mutable (puede cambiar de valor)
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // constantes
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of x is: {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    // puede declarar una nueva variable con el mismo nombre que una variable anterior.
    // los Rustaceans dicen que la primera variable es ocultada por la segunda, lo que significa que la segunda variable es lo que el compilador verá cuando use el nombre de la variable
    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");
}
