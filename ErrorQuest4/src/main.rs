#[derive(Debug)]
struct Room {
    name: String,
    danger_level: u32,
    treasure: u32,
    cleared: bool,
}

#[derive(Debug)]
struct Explorer {
    name: String,
    health: i32,
    gold: u32,
}

#[derive(Debug)]
struct Dungeon {
    rooms: Vec<Room>,
}

#[derive(Debug)]
enum DungeonError {
    RoomNotFound,
	ExplorerDead,
	RoomAlreadyCleared,
	TooDangerous,
}

impl Dungeon {
    fn new() -> Dungeon {
        Dungeon { rooms: Vec::new() }
    }

    fn add_room(&mut self, room: Room) {
        self.rooms.push(room)
    }

    fn enter_room(&mut self, explorer: &mut Explorer, room_name: &str) -> Result<(), DungeonError> {
        if explorer.health <= 0 { return Err(DungeonError::ExplorerDead) };

        let room_options = self.rooms.iter_mut().find(|room| room.name == room_name);

        match room_options {
            Some(room) => {
                if room.cleared == true {
                    Err(DungeonError::RoomAlreadyCleared)
                } else if room.danger_level > explorer.health as u32 {
                    Err(DungeonError::TooDangerous)
                } else {
                    explorer.health -= room.danger_level as i32;
                    explorer.gold += room.treasure;
                    room.cleared = true;
                    Ok(())
                }
            }
            None => { Err(DungeonError::RoomNotFound) }
        }
    }
}

fn main() {
    let mut explorer = Explorer { name: "Rusty".to_string(), health: 100, gold: 0, };

    let mut dungeon = Dungeon::new();

    dungeon.add_room(Room {
        name: "KorgRoom".to_string(),
        danger_level: 122,
        treasure: 224,
        cleared: false,
    });

    dungeon.add_room(Room {
        name: "Room303".to_string(),
        danger_level: 30,
        treasure: 67,
        cleared: false,
    });

    match dungeon.enter_room(&mut explorer, "Room303") {
        Ok(_) => println!("Salle explorée !"),
        Err(e) => println!("Erreur: {:?}", e),
    }

    println!("Vie: {}, Gold: {}", explorer.health, explorer.gold);
}

