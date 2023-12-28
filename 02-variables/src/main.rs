fn main() {
    // variables inmutables
    let cat: &str = "Kitty";
    println!("Hey {cat}, what's going on?");

    let cat_age: u8 = 5;
    println!("{cat_age}");

    // variables mutables
    let mut cat2: &str = "Lolin";
    let mut cat_age2: u8 = 3;
    println!("Hey {cat2}, what's going on?");
    println!("{cat_age2}");

    cat2 = "Kiwi";
    cat_age2 = 4;

    println!("Hey {cat2}, what's going on?");
    println!("{}", cat_age2);

    println!("===============================");
    println!("What's your name?");
    let mut name: String = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    name = name.trim().to_string();

    println!("Where your live?");
    let mut country: String = String::new();
    std::io::stdin().read_line(&mut country).unwrap();
    country = country.trim().to_string();

    println!("Hey {}, what's going on?", name);
    println!("{} is from {}", name, country);
}
