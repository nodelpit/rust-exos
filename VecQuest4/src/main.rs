// Représente un objet d'inventaire
struct Item {
    name: String,
    quantity: i32,
    broken: bool,
}

// Représente une action sur un objet
enum Action {
    Use(i32), // Consomme une quantity
    Repair,
    Discard,
}

impl Item {
    fn apply(&mut self, action: &Action) {
        match action {
            Action::Use(u) => {
                self.quantity -= u;
                if self.quantity <= 0 {
                    self.broken = true;
                }
            }

            Action::Repair => {
                if self.quantity > 0 { self.broken = false; } else {}
            }

            Action::Discard => {
                self.quantity = 0;
                self.broken = true;
            }
        }
    }
}

fn main() {
    let mut item_vecteur: Vec<Item> = vec![
        Item {
            name: "Drone".to_string(),
            quantity: 2,
            broken: false,
        },

        Item {
            name: "Camera".to_string(),
            broken: true,
            quantity: 1,
        },

        Item {
            name: "propeller".to_string(),
            quantity: 4,
            broken: true,
        },
    ];

    let action_vecteur: Vec<Action> = vec![
        Action::Use(1),
        Action::Repair,
        Action::Discard,
        ];

    for item in item_vecteur.iter_mut() {
        for action in &action_vecteur {
            item.apply(action);
            println!("{}: quantity={  }, broken={ }", item.name, item.quantity, item.broken);
        }
    }
}
