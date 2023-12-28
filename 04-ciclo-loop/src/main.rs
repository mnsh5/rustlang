fn main() {
    let num_1 = 3;
    let num_2 = 2;
    let suma = num_1 + num_2;
    println!("La suma es: {suma}");

    loop {
        let mut user_sum = String::new();
        std::io::stdin().read_line(&mut user_sum).unwrap();

        let user_sum: i16 = user_sum.trim().parse().unwrap();

        if user_sum == suma {
            println!("Awesome... 🐎");
        } else {
            println!("Oops... 🦧");
        }
    }
}
