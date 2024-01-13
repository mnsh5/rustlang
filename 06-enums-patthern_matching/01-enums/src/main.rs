// Definiendo un Enum
// Los structs te permiten agrupar campos relacionados y datos, como un Rectángulo con su ancho y largo.
// Por otro lado, las enumeraciones te permiten decir que un valor es uno de un conjunto de posibles valores.

// Digamos que tenemos que trabajar con direcciones IP.
// Actualmente, dos estándares son los que se usan para direcciones IP: la versión cuatro y la versión seis.
// Como estos son los únicos posibles tipos de direcciones IP que nuestro programa encontrará, podemos enumerar todas las variantes posibles, de donde viene el nombre de enumeración.

// Definimos una enumeración IpAddrKind enumerando los posibles tipos de direcciones IP, V4 y V6
// IpAddrKind ahora es un tipo de datos personalizado que podemos usar en otras partes de nuestro código
// enum IpAddrKind {
//     V4,
//     V6,
// }
// enum IpAddr {
//     V4(String),
//     V6(String),
// }

enum Message {
    Write(String),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Write(s) => println!("Azure Kubernetes Service: {}", s),
        }
    }
}

fn main() {
    // Valores Enum
    // Podemos crear instancias de cada una de las dos variantes de IpAddrKind de esta manera:
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // podemos llamar a esta función con cualquiera de las variantes:
    // route(IpAddrKind::V4);
    // route(IpAddrKind::V6);

    // let home = IpAddr::V4(String::from("127.0.0.1"));

    // let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("AKS"));
    m.call();
}

// Definimos una función que tome cualquier IpAddrKind
// fn route(ip_kind: IpAddrKind) {}
