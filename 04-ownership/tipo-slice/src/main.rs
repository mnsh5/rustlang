// El Tipo Slice
// Los Slices te permiten referenciar a una secuencia contigua de elementos en una colección en lugar de la colección completa.
// Un slice es una especie de referencia, por lo que no tiene ownership.

fn main() {
    // String Slices
    // Un string slice es una referencia de parte de un String, y se ve algo como esto:
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{}", hello);
    println!("{}", world);
    //  Creamos slices usando un rango dentro de corchetes, especificando [starting_index..ending_index]

    // Con la sintaxis de rango .. de Rust, si queremos comenzar en el índice 0, podemos dejar el valor antes de los dos puntos.
    // En otras palabras, estos son iguales:
    let s = String::from("hello");
    let len = s.len();
    let slice1 = &s[3..len];
    let slice2 = &s[3..];
    println!("{}", slice1);
    println!("{}", slice2);

    // También podemos omitir ambos valores para tomar un slice de todo el string.
    // Entonces estos son iguales:
    let slice3 = &s[0..len];
    let slice4 = &s[..];
    println!("{}", slice3);
    println!("{}", slice4);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
