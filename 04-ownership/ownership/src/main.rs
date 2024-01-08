fn main() {
    let mut s = String::from("hola");
    s.push_str(", mundo!"); // push_str() agrega un literal a un String
    println!("{}", s); // Esto imprime "hola, mundo!"
}
