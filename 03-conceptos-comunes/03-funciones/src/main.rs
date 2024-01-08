fn main() {
    println!("Hello, world!");

    another_function();
    another_function_two(5);
    print_labeled_measurement(5, 'h');

    let x = five();
    println!("The value of x is: {x}");

    let y = plus_one(7);
    println!("The value of x is: {y}");
}

fn another_function() {
    println!("Another function.");
}

fn another_function_two(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// Funciones con valores de retorno
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
