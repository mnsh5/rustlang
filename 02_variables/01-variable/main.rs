fn main() {
    // Una variable solo se puede utilizar si se ha inicializado.
    let x: i32 = 5;
    let y: i32 = 3;

    assert_eq!(x, 5);
    assert_eq!(y, 3);
    println!("Success!");

    // Se utiliza mut para marcar una variable como mutable
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 3;
    println!("The value of x is: {}", x);
}
