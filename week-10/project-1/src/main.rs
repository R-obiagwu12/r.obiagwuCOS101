/// Define a structure for Laptop
struct Laptop {
    brand: String,
    price: u32,
    quantity: u32,
}

impl Laptop {
    // Method to calculate total cost
    fn total_cost(&self) -> u32 {
        self.price * self.quantity
    }
    
    // Method to calculate cost for specific purchase quantity
    fn purchase_cost(&self, purchase_qty: u32) -> u32 {
        self.price * purchase_qty
    }
}

fn main() {
    // Create laptop inventory
    let laptops = [
        Laptop { brand: String::from("HP"), price: 650_000, quantity: 10 },
        Laptop { brand: String::from("IBM"), price: 755_000, quantity: 6 },
        Laptop { brand: String::from("Toshiba"), price: 550_000, quantity: 10 },
        Laptop { brand: String::from("Dell"), price: 850_000, quantity: 4 },
    ];
    
    // Customer purchases 3 from each brand
    let purchase_per_brand = 3;
    let mut total_cost = 0;
    
    println!("MR OGLEIFUNA'S LAPTOP PURCHASE CALCULATION");
    println!("===========================================\n");
    println!("Customer purchases {} laptops from each brand:", purchase_per_brand);
    println!("-----------------------------------------------");
    
    for laptop in &laptops {
        let cost = laptop.purchase_cost(purchase_per_brand);
        total_cost += cost;
        
        println!("{} laptop: ₦{} × {} = ₦{}", 
                 laptop.brand, 
                 laptop.price, 
                 purchase_per_brand, 
                 cost);
    }
    
    println!("-----------------------------------------------");
    println!("TOTAL COST: ₦{}", total_cost);
    println!("TOTAL LAPTOPS: {}", purchase_per_brand * laptops.len() as u32);
}