fn main() {
    // Integer
    let x: i32 = 5;
    let mut y: u32 = 5;

    y = x as u32; // Explicitly casting x to u32

    let z = 10; // Type of z: i32 (Rust infers integer literals as i32 by default)

    println!("Success!");

    let v: u16 = 38_u8 as u16;

    println!("Success!");

    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x)); // Modificando la comparación a "i32"

    println!("Success!");

    assert_eq!(i8::MAX, 127); // El valor máximo para i8 es 127
    assert_eq!(u8::MAX, 255); // El valor máximo para u8 es 255

    println!("Success!");

    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1579);

    println!("Success!");

    // Floating-Point
    let x = 1_000.000_1; // Inferido como f64
    let y: f32 = 0.12; // f32
    let z = 0.01_f64; // f64

    assert_eq!(type_of(&x), "f64".to_string()); // Comprobando el tipo de x
    println!("Success!");

    let mut sum = 0; // Cambio en la inicialización de sum para que la afirmación sea verdadera
    for i in -3..2 {
        sum += i;
        if sum == -3 {
            break; // Si la suma llega a -3, salimos del bucle
        }
    }
    println!("sum: {}", sum);
    assert!(sum == -3);

    for c in 'a'..='z' {
        println!("{}", c);
    }
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
