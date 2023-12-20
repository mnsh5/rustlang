fn main() {
    // Destructuring
    // 🌟🌟 Podemos usar un patrón con letpara desestructurar una tupla para separar variables
    let (mut x, y) = (1, 3);
    x += 2;
    assert_eq!(x, 3);
    assert_eq!(y, 3);
    println!("Success!");

    // Tareas de desestructuración
    let (x, y);
    (x, ..) = (3, 4);
    [.., y] = [1, 2];
    assert_eq!([x, y], [3, 2]);

    println!("Success!");
}
