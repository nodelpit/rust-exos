#[derive(Debug)]
struct Response {
    status: u16,
    body: String,
}

struct HelloHandler {
    body: String,
}

trait Handler {
    fn call(&self) -> Response;
}

fn execute<T: Handler>(handler: T) -> Response {
    handler.call()
}

impl Handler for HelloHandler {
    fn call(&self) -> Response {
        Response {
            status: 200,
            body: self.body.clone(),
        }
    }
}

// Toutes les fonctions ou closures qui retournent Response utiliseront le trait handler 🧠
impl<F: Fn() -> Response> Handler for F {
    fn call(&self) -> Response {
        self()
    }
}

// impl<F> Handler for F
// where
//     F: Fn() -> Response,
//     {
//         fn call(&self) -> Response {
//             self()
//         }
//     }

fn my_function() -> Response {
    Response {
        status: 201,
        body: String::from("Hello from Function"),
    }
}

fn main() {
    // Struct handler
    let r1 = execute(HelloHandler {
        body: String::from("Hello from Struct !"),
    });

    // Function handler
    let r2 = execute(my_function);

    // Closure handler
    let r3 = execute(|| Response {
        status: 202,
        body: String::from("Hello from Closure"),
    });

    println!("{:?}", r1);
    println!("{:?}", r2);
    println!("{:?}", r3);
}
