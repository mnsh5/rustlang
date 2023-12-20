fn main() {
    // Una variable solo se puede utilizar si se ha inicializado.
    let x: i32 = 5;
    let y: i32 = 3;

    assert_eq!(x, 5);
    assert_eq!(y, 3);
    println!("Success!");

    // Se utiliza mut para marcar una variable como mutable
    let mut x = 5;
    println!("The value of x is: {}", x); // Changed {x} to {}
    x = 3;
    println!("The value of x is: {}", x); // Changed {x} to {}

    // constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("THREE_HOURS_IN_SECONDS: {}", THREE_HOURS_IN_SECONDS); // Added {}

    // Shadowing
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {}", y);
    }
    println!("The value of y is: {}", y);
}
