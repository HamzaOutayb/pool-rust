use std::iter::Product;

#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}
impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub reciept: Vec<f32>,
}

impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: Vec::new(),
            reciept: Vec::new(),
        }
    }
    pub fn insert_item(&mut self, s: &Store, ele: String) {
        for product in s.products.iter() {
            if product.0 == ele {
                self.items.push((product.0.clone(), product.1));
                self.reciept.push(product.1);
            }
        }
    }
    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut sorted = self.reciept.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let total: f32 = sorted.iter().sum();
        let min: f32 = sorted[..(sorted.len()/3 as usize)].iter().sum();

        sorted = sorted
        .iter()
        .map(|v| format!("{:.2}", v - (min*(v/total))).parse::<f32>().unwrap())
        .collect();

        self.reciept = sorted.clone();
        sorted
    }
}


// 20 30 50