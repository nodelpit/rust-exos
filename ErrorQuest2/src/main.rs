#[derive(Debug)]
struct Potion {
    name: String,
    potency: i32,
    charges: u32,
    ingredient: String,
    stable: bool,
}

struct PotionBag {
    potions: Vec<Potion>,
}

#[derive(Debug)]
enum PotionError {
    InvalidPotency,
    InvalidCharges,
    ProhibitedIngredient,
    PotionUnstable,
    NoChargesLeft,
    InvalidIndex,
}

impl Potion {
    fn craft(name: &str, potency: i32, charges: u32, ingredient: &str, stable: bool) -> Result<Potion, PotionError> {
        if potency <= 0 {
            Err(PotionError::InvalidPotency)
        } else if charges == 0 {
            Err(PotionError::InvalidCharges)
        } else if ingredient == "water" || ingredient == "mud" {
            Err(PotionError::ProhibitedIngredient)
        } else { Ok(Potion {
            name: name.to_string(),
            potency: potency,
            charges: charges,
            ingredient: ingredient.to_string(),
            stable: stable,
        }) }
    }

    fn use_charge(&mut self) -> Result<(), PotionError> {
        if self.charges == 0 {
            Err(PotionError::NoChargesLeft)
        } else {
            self.charges -= 1;
            Ok(())
        }
    }

    fn activate(&mut self) -> Result<i32, PotionError> {
        if !self.stable {
            Err(PotionError::PotionUnstable)
        } else {
            self.use_charge()?;
            Ok(self.potency)
        }
    }
}

impl PotionBag {
    fn new() -> PotionBag {
        PotionBag { potions: Vec::new() }
    }

    fn add_potion(&mut self, potion: Potion) {
        self.potions.push(potion)
   }

   fn activate_potion(&mut self, index: usize) -> Result<i32, PotionError> {
        match self.potions.get_mut(index) {
            Some(potion) => potion.activate(),
            None => Err(PotionError::InvalidIndex),
        }
   }
}


fn main() {
    let mut potion_bag = PotionBag::new();

    let potion1 = Potion::craft("Magic shield", 15, 3, "Blue flower", true);
    let potion2 = Potion::craft("Fire ball", 30, 3, "Dragon eye", true);

    match potion1 {
        Ok(potion) => potion_bag.add_potion(potion),
        Err(e) => println!("Impossible d'ajouter la potion1 dans votre sac: {:?}", e),
    }

    match potion2 {
        Ok(potion) => potion_bag.add_potion(potion),
        Err(e) => println!("Impossible d'ajouter la potion2 dans votre sac: {:?}", e),
    }

    match potion_bag.activate_potion(0) {
        Ok(potency) => println!("Potion 0 activée ! Potency: {}", potency),
        Err(e) => println!("Erreur activation de la potion2: {:?}", e),
    }

    match potion_bag.activate_potion(1) {
        Ok(potency) => println!("Potion 1 activée ! Potency: {}", potency),
        Err(e) => println!("Erreur activation de la potion1: {:?}", e),
    }
}
