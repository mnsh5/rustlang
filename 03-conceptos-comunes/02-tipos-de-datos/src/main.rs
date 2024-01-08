fn main() {
    // Tipos Escalares
    // Un tipo escalar representa un solo valor
    // Rust tiene cuatro tipos escalares principales: enteros, números de punto flotante, booleanos y caracteres
    // Puede reconocerlos de otros lenguajes de programación

    // Tipo Integers
    // Un entero es un número sin componente fraccionario

    /*
    Los números con signo pueden representar valores positivos o negativos
    los números sin signo representan solo valores no negativos

    Tamaño	Signed	Unsigned
    8-bit	i8	    u8
    16-bit	i16	    u16
    32-bit	i32	    u32
    64-bit	i64	    u64
    128-bit	i128	u128
    arch	isize	usize
    */

    // Float
    // Los tipos de punto flotante de Rust son f32 y f64, que tienen 32 bits y 64 bits de tamaño, respectivamente
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("{x}\n{y}");

    // Operaciones numéricas
    // addition
    let sum = 5 + 10;
    println!("{sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("{difference}");

    // multiplication
    let product = 4 * 30;
    println!("{product}");

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    println!("{quotient}");
    println!("{truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("{remainder}");

    // Boolean
    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("{t}");
    println!("{f}");

    // Char
    // especificamos literales char con comillas simples, en oposición a literales de cadena, que usan comillas dobles
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("{c}");
    println!("{z}");
    println!("{heart_eyed_cat}");

    // Tipos compuestos
    // Pueden agrupar múltiples valores en un solo tipo. Rust tiene dos tipos compuestos primitivos: tuplas y matrices.
    let tup1: (i32, f64, u8) = (500, 6.4, 1);

    // Desestructurar la tupla
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    // También podemos acceder directamente a un elemento de la tupla usando un punto (.) seguido del índice del valor que queremos acceder
    let five_hundred = tup1.0;
    let six_point_four = tup1.1;
    let one = tup1.2;
    println!("The value of six_point_four is: {six_point_four}");
    println!("The value of five_hundred is: {five_hundred}");
    println!("The value of one is: {one}");

    // Array
    // let arr2: [i32; 5] = [1, 2, 3, 4, 5];
    let arr = [1, 2, 3, 4, 5];
    println!("{:?}", arr);
    println!("{}", arr[0]);
    println!("{}", arr[1]);
    println!("{}", arr[2]);
    for (i, &v) in arr.iter().enumerate() {
        println!("Array Index: {} - Array Element: {}", i, v);
    }
    println!("Array Length: {}", arr.len());
}
