// Magic shop inventory
// un petit système de boutique magique qui vend des objets (potions, artefacts…) et gère des erreurs avec Result.

#[derive(Debug)]
struct MagicItem {
    name: String,
    price: u32,
    stock: u32,
    cursed: bool,
}

#[derive(Debug)]
struct MagicShop {
    inventory: Vec<MagicItem>,
    gold: u32,
}

#[derive(Debug)]
enum ShopError {
    InvalidPrice,
    InvalidStock,
    ItemNotFound,
    OutOfStock,
    NotEnoughGold,
    ItemIsCursed,
}

impl MagicItem {
    fn craft(name: &str, price: u32, stock: u32, cursed: bool) -> Result<MagicItem, ShopError> {
        if price == 0 {
            Err(ShopError::InvalidPrice)
        } else if stock == 0 {
            Err(ShopError::InvalidStock)
        } else { Ok(MagicItem {
            name: name.to_string(),
            price: price,
            stock: stock,
            cursed: cursed,
        } ) }
    }
}

impl MagicShop {
    fn new(gold: u32) -> MagicShop {
        MagicShop { inventory: Vec::new(), gold }
    }

    fn add_item(&mut self, item: MagicItem) {
        self.inventory.push(item)
    }

    fn buy_item(&mut self, name: &str) -> Result<(), ShopError> {
        match self.inventory.iter_mut().find(|item| item.name == name ) {
            Some(item) => {
                if item.stock == 0 {
                    Err(ShopError::OutOfStock)
                } else if item.cursed == true {
                    Err(ShopError::ItemIsCursed)
                } else if self.gold < item.price {
                    Err(ShopError::NotEnoughGold)
                } else {
                    item.stock -= 1;
                    self.gold -= item.price;
                    Ok(())
                }
            }
            None => Err(ShopError::ItemNotFound),
        }
    }

    fn show_inventory(&self) {
        for (i, item) in self.inventory.iter().enumerate() {
            println!(
                "{} - {} | price: {} | stock: {} | cursed: {}",
                i + 1,
                item.name,
                item.price,
                item.stock,
                item.cursed,
            );
        }
    }
}


fn main() {
    let mut magic_shop = MagicShop::new(60);

    let item1 = MagicItem::craft("Monster trap", 30, 20, true);
    let item2 = MagicItem::craft("Invisibility potion", 60, 0, false);
    let item3 = MagicItem::craft("Mithril knife", 5, 10, false);

    match item1 {
        Ok(item) => magic_shop.add_item(item),
        Err(e) => println!("Erreur craft item1: {:?}", e),
    }

    match item2 {
        Ok(item) => magic_shop.add_item(item),
        Err(e) => println!("Erreur craft item2: {:?}", e),
    }

    match item3 {
        Ok(item) => magic_shop.add_item(item),
        Err(e) => println!("Erreur craft item3: {:?}", e),
    }

    println!("--- Inventory ---");
    magic_shop.show_inventory();

    println!("--- Achat ---");
    match magic_shop.buy_item("Mithril knife") {
        Ok(_) => println!("Achat réussi !"),
        Err(e) => println!("Erreur achat: {:?}", e),
    }

    println!("--- Inventory après achat ---");
    magic_shop.show_inventory();
}
