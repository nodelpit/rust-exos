#[derive(Debug)]
struct Response {
    status: u16,
    body: String,
}

#[derive(Debug)]
struct AppError {
    message: String,
}

trait IntoResponse {
    fn into_response(self) -> Response;
}

fn handle<T: IntoResponse>(value: T) -> Response {
    value.into_response()
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        Response {
            status: 500,
            body: self.message,
        }
    }
}

impl IntoResponse for String {
    fn into_response(self) -> Response {
        Response {
            status: 200,
            body: self,
        }
    }
}

fn main() {
    let ok = handle(String::from("success"));
    println!("{:?}", ok);

    let error = handle(AppError {
        message: String::from("Something went wrong.."),
    });
    println!("{:?}", error);
}
