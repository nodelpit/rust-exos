#[derive(Debug)]
struct Data {
    value: String,
}

impl Data {
    fn new(s: &str) -> Data {
        Data {
            value: s.to_string(),
        }
    }
}

fn consume(param: Data) {
    println!("{}", param.value);
}

fn main() {
    let data = Data::new("rust");
    consume(data);

    println!("{:?}", data);
}