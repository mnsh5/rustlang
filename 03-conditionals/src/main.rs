fn main() {
    println!("Que pastilla tomaras? roja o azul?");

    let mut opcion: String = String::new();
    std::io::stdin().read_line(&mut opcion).unwrap();
    let pastilla: &str = opcion.trim();

    if pastilla == "roja" {
        println!("Neo, conoces un Carpincho?");
    } else if pastilla == "azul" {
        println!("Neo, la vida es bella");
    } else {
        println!("Neo, llueve en Buenos Aires");
    }
}
