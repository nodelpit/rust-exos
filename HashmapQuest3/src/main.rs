use std::collections::HashMap;

struct Inventory {
    data: HashMap<String, i32>,
}

impl Inventory {
    fn new() -> Self {
        Self {
            data: HashMap::new()
        }
    }

    fn add_product(&mut self, name: &str, quantity: i32) {
        self.data.insert(name.to_string(), quantity);
    }

    fn update_stock(&mut self, name: &str, delta: i32) {
        if let Some(s) = self.data.get_mut(name) {
            *s += delta;
        } else {
            println!("Le produit '{}' n'a pas été trouvé", name);
        }
    }

    fn has_product(&self, name: &str) -> bool {
        self.data.contains_key(name)
    }

    fn display(&self) {
        for (k, v) in &self.data {
            println!("{} -> {}", k, v);
        }
    }
}

fn main() {
    let mut data = Inventory::new();

    data.add_product("server", 5);
    data.add_product("keyboard", 10);
    data.add_product("led", 7);
    data.add_product("cable", 15);

    data.update_stock("server", 7);

    data.update_stock("controller", 3);

    data.display();

    println!("{:?}", data.has_product("cable"));
}
