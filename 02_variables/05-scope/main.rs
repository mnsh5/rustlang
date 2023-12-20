// Un scope es el rango dentro del programa para el cual el artículo es válido
fn main() {
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    // La variable "y" no está disponible aquí, por lo que el código no compilará
    // println!("The value of x is {} and value of y is {}", x, y);
}
