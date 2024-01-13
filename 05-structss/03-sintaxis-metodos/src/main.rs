// Sintaxis de Métodos
// Los métodos son similares a las funciones: los declaramos con la palabra clave fn y un nombre, pueden tener parámetros
// y un valor de retorno, y contienen alguno código que se ejecuta cuando el método es llamado desde otro lugar.
// A diferencia de las funciones, los métodos se definen dentro del contexto de una estructura
// y su primer parámetro siempre es self, que representa la instancia de la estructura en la que se está llamando al método

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Iniciamos un bloque impl (implementación). Todo lo que esté dentro de este bloque impl estará asociado al tipo Rectangle
impl Rectangle {
    // Siempre todos los metodos van dentro de un Impl

    // metodo area con el primero parametro self que siempre lo lleva
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // metodo width
    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    // iniciamos la instancia de Rectangle (instancia que viene del struct y del impl)
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
