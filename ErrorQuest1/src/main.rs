#[derive(Debug)]
struct Potion {
    name: String,
    power: i32,
    charges: u32,
    ingredient: String,
}

#[derive(Debug)]
enum PotionError {
    ProhibitedIngredient,
    LowPower,
    DangerousPower,
    InvalidCharges,
}

impl Potion {
    fn craft(name: &str, power: i32, charges: u32, ingredient: &str) -> Result<Potion, PotionError> {
        if power <= 0 {
            Err(PotionError::LowPower)
        } else if charges == 0 {
            Err(PotionError::InvalidCharges)
        } else if ingredient == "water" {
            Err(PotionError::ProhibitedIngredient)
        } else if power > 100 {
            Err(PotionError::DangerousPower)
        } else { Ok(Potion {
            name: name.to_string(),
            power: power,
            charges: charges,
            ingredient: ingredient.to_string(),
        },) }
    }

    fn use_charge(&mut self) -> Result<(), PotionError> {
        if self.charges == 0 {
            Err(PotionError::InvalidCharges)
        } else {
            self.charges -= 1;
            Ok(())
        }
    }

    fn drink(&mut self,) -> Result<i32, PotionError> {
        if self.power > 100 {
            Err(PotionError::DangerousPower)
        } else {
            self.use_charge()?;
            Ok(self.power)
        }
    }
}

fn main() {
    let potion_result = Potion::craft("vital_potion", 20, 5, "red_flower");

    match potion_result {
        Ok(mut potion) => {
            match potion.drink() {
                Ok(power) => println!("Potion utilisée ! Puissance de la potion: {}", power),
                Err(e) => println!("Erreur en utilisant la potion: {:?}", e),
            }
        },
        Err(e) => println!("Erreur lors de la création de la potion: {:?}", e),
    }
}
