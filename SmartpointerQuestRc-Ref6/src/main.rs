use std::rc::Rc;
use std::cell::RefCell;

struct NotificationCenter {
    notifications: Vec<String>,
}

impl NotificationCenter {
    fn new() -> NotificationCenter {
        NotificationCenter {
            notifications: Vec::new()
        }
    }

    fn add(&mut self, message: &str) {
        self.notifications.push(message.to_string());
    }

    fn list(&self) {
        for notification in self.notifications.iter() {
            println!(
                "{:?}", notification
            );
        }
    }

    fn consume(&mut self) {
        self.notifications.pop();
    }
}

fn auth_service(param: Rc<RefCell<NotificationCenter>>) {
    param.borrow_mut().add("User authenticated");
}

fn message_service(param: Rc<RefCell<NotificationCenter>>) {
    param.borrow_mut().add("New message");
}

fn error_service(param: Rc<RefCell<NotificationCenter>>) {
    param.borrow_mut().add("Error: Invalid input");
}

fn main() {
    let notifications_center = Rc::new(RefCell::new(
        NotificationCenter::new()
    ));

    auth_service(Rc::clone(&notifications_center));
    message_service(Rc::clone(&notifications_center));
    error_service(Rc::clone(&notifications_center));

    notifications_center.borrow().list();
    println!("\n");

    notifications_center.borrow_mut().consume();

    notifications_center.borrow().list();
}
