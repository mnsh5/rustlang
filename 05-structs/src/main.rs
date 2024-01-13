// Usando Structs para Estructurar Datos Relacionados
// Un struct, o estructura, es un tipo de dato personalizado que te permite empaquetar y nombrar múltiples valores relacionados que forman un grupo significativo
// Si estás familiarizado con un lenguaje orientado a objetos, un struct es como los atributos de un objeto
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Usando Structs de Tuplas sin Campos Nombrados para Crear Diferentes Tipos
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Structs de Unidad sin Campos
struct AlwaysEqual;

fn main() {
    // Para usar un struct después de haberlo definido, creamos una instancia de ese struct especificando valores concretos para cada uno de los campos
    let mut user = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );

    // Creando Instancias de Otras Instancias con Sintaxis de Struct Update
    let user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // Usando la sintaxis de struct update, podemos lograr el mismo efecto con menos código
    let user4 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // Usando Structs de Tuplas sin Campos Nombrados para Crear Diferentes Tipos
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Structs de Unidad sin Campos
    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
