fn main() {
    for c in 'a'..='z' {
        println!("{}", c);
    }

    let mut sum = 0;
    for i in 1..=10 {
        // Utilizamos 1..=10 para incluir el 10 en el rango
        sum += i;
    }
    println!("sum: {}", sum);
    assert!(sum == 55); // La suma de los números del 1 al 10 es 55
}
