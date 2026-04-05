enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn print_list(list: &List) {
    match list {
        List::Cons(value, next) => {
            println!("{}", value);
            print_list(next)

        }
        List::Nil => println!("Stop recursivité")
    }
}

fn main() {
    let new_list = List::Cons(10, Box::new(List::Cons(20, Box::new(List::Cons(30, Box::new(List::Nil))))));
    print_list(&new_list);
}
