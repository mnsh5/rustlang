fn main() {
    // Definimos un array de razas de gatos
    let cat_breeds = ["Siames", "Persa", "Maine Coon", "Bengala", "Sphynx"];

    // Accedemos e imprimimos algunas razas de gatos
    println!("Razas de gatos populares:");
    println!("1. {}", cat_breeds[0]); // Accediendo al primer elemento
    println!("2. {}", cat_breeds[1]); // Accediendo al segundo elemento

    for &breed in cat_breeds.iter() {
        if breed == "Sphynx" {
            println!("{breed} is my favorite cat! 🐈");
            continue;
        }

        println!("{breed}")
    }

    let nums = [1, 2, 1];
    let mut sum = 0;
    for &num in nums.iter() {
        sum += num;
    }
    println!("Sum: {sum}");
}
