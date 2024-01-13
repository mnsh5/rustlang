fn main() {
    // podemos usar llaves para crear un nuevo ámbito, permitiendo múltiples referencias mutables, solo no simultáneas
    let mut s = String::from("code-server");

    {
        let r1 = &mut s;
        println!("r1: {r1}");
    } // r1 se sale de su ámbito aquí, por lo que no hay problema
      // si creamos otra referencia mutable

    let r2 = &mut s;
    println!("r2: {r2}");

    let mut s = String::from("hello");

    let a1 = &s; // no hay problema
    let a2 = &s; // no hay problema
    println!("{} y {}", a1, a2);
    // variables r1 y r2 no se usaran más a partir de aquí

    let a3 = &mut s; // no hay problema
    println!("{}", a3);
}
