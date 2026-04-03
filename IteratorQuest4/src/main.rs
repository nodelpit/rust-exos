struct Order {
    name: String,
    price: f32,
    status: OrderStatus,
}

struct OrderManager {
    orders: Vec<Order>,
}

#[derive(PartialEq, Debug)]
enum OrderStatus {
    Pending,
    Completed,
    Cancelled,
}

impl Order {
    fn new(name: &str, price: f32) -> Order {
        Order {
            name: name.to_string(),
            price: price,
            status: OrderStatus::Pending,
        }
    }

    fn complete(&mut self) {
        self.status = OrderStatus::Completed
    }

    fn cancel(&mut self) {
        self.status = OrderStatus::Cancelled
    }

    fn is_available(&self) -> bool {
        self.status == OrderStatus::Completed
    }
}

impl OrderManager {
    fn new() -> OrderManager {
        OrderManager { orders: Vec::new() }
    }

    fn add_order(&mut self, order: Order) {
        self.orders.push(order)
    }

    fn list_orders(&self) {
        for (i, order) in self.orders.iter().enumerate() {
            println!(
                "{} - {}, prix: {}, statut: {:?}",
                i + 1,
                order.name,
                order.price,
                order.status,
            )
        }
    }

    fn completed_orders(&self) -> Vec<&Order> {
        self.orders
                .iter()
                .filter(|order| order.is_available())
                .collect()
    }

    fn completed_prices(&self) -> Vec<f32> {
        self.orders
                .iter()
                .filter(|order| order.is_available())
                .map(|order| order.price)
                .collect()
    }

    fn total_completed_price(&self) -> f32 {
        self.orders
                .iter()
                .filter(|order| order.is_available())
                .map(|order| order.price)
                .sum()
    }
}

fn main() {
    let mut order_manager = OrderManager::new();

    let order1 = Order::new("Elektron Syntakt", 998.0);
    let order2 = Order::new("Cyclone TT303", 274.0);
    let order3 = Order::new("Yamah HS7", 450.0);

    order_manager.add_order(order1);
    order_manager.add_order(order2);
    order_manager.add_order(order3);


    order_manager.list_orders();
}
