#[derive(Debug)]
struct Data {
    value: String,
}

impl Data {
    fn new(s: &str) -> Data {
        Data { value: s.to_string() }
    }
}

fn consume_and_return(param: Data) -> Data {
    println!("{}", param.value);
    Data {
        value: format!("{} and Axum", param.value),
    }
}

fn main() {
    let data = Data::new("rust");
    let data = consume_and_return(data);
    println!("{:?}", data);
}

// // Ici, la fonction consume_and_return prend l'ownership de `param`.
// Elle peut donc modifier sa valeur librement et la retourner ensuite.
// On réassigne le résultat à `data` pour récupérer la valeur modifiée.
// 
// Rust empêche toute utilisation de `data` après qu'elle a été déplacée,
// donc sans réaffectation, on ne pourrait plus l'utiliser.
// 
// Cette méthode illustre comment travailler uniquement avec l’ownership,
// sans utiliser de borrowing (`&` ou `&mut`).
// C’est correct pour de petites structures, mais moins efficace si la struct est grande.
// Dans la pratique, pour modifier `data` directement, on utiliserait un emprunt mutable.