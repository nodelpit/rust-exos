enum Event {
    Add(i32),
    Remove(i32),
    Reset,
}

struct Counter {
    value: i32,
}

impl Counter {
    fn apply (&mut self, event: &Event, index: usize) {
        match event {
            Event::Add(a) => { self.value += a },
            Event::Remove(rem) => { self.value -= rem },
            Event::Reset => self.value = 0,
        }
    }
}

fn main() {
    let vecteur = vec![
        Event::Add(10),
        Event::Add(10),
        Event::Add(5),
        Event::Remove(5),
        Event::Reset,
    ];

    let mut counter = Counter {
        value: 0,
    };

    for (index, event) in vecteur.iter().enumerate() {
        counter.apply(event, index);

        if index % 2 == 0 {
            println!("Valeur actuelle: {}", counter.value);
        }
    }
}
