use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};
use near_sdk::collections::Vector;

// hotel mgt
// visitors
//  -> reserve a table
//  -> order food
//  -> access menu
//  -> pay

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Food {
    menu_item: String,
    amount: i16,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Visitor {
    seat_number: i16,
    is_served: bool,
    order: Vec<Food>,
    food: Vec<Food>,
}
impl Default for Visitor {
    fn default() -> Self {
        Visitor {
            seat_number: 0,
            is_served: false,
            order: vec![], //Vector::new(b"r".to_vec()),
            food: vec![],  //Vector::new(b"r".to_vec()),
        }
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Hotel {
    visitors: Vec<Visitor>,
    food: Vec<Food>,
}
#[near_bindgen]
impl Hotel {
    #[init]
    #[private]
    pub fn new() -> Self {
        // let mut food_types =  Vec::new(); //Vector::new(b"r".to_vec());
        let fish = Food {
            menu_item: "fish".to_string(),
            amount: 300,
        };
        let chicken = Food {
            menu_item: "chicken".to_string(),
            amount: 400,
        };
        let mutton = Food {
            menu_item: "mutton".to_string(),
            amount: 500,
        };
        Self {
            visitors: vec![], //Vector::new(b"r".to_vec()),
            food: vec![fish, chicken, mutton],
        }
    }

    // new visitor
    pub fn new_visitor(&mut self, seat_number: i16) {
        let vst = Visitor {
            seat_number: seat_number,
            is_served: false,
            order: vec![], //Vector::new(b"r".to_vec()),
            food: vec![],  //Vector::new(b"r".to_vec()),
        };

        self.visitors.push(vst);
    }

    //new_order
    pub fn new_order(&mut self, seat_number: i16, menu_item: String) {
        let mut food_item: Option<&Food> = None;
        for food in self.food.iter() {
            if food.menu_item == menu_item {
                food_item = Some(food);
            }
        }

        match food_item {
            Some(x) => {
                for visitor in self.visitors.iter_mut() {
                    if visitor.seat_number == seat_number {
                        let tmp = Food {
                            menu_item: x.menu_item.clone(),
                            amount: x.amount.clone(),
                        };
                        visitor.order.push(tmp);
                    }
                }
            }
            None => {
                let food = format!("The food you ordered {} does not exist ", menu_item);
                env::log_str(food.as_str());
            }
        }
    }

   
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;

    

    // TESTS HERE

    #[test]
    pub fn new_order() {
        let mut order1 = Hotel::new();
        order1.new_visitor(5); // index 0
        order1.new_order(5, "chicken".to_string()); // index 1

        // assert_eq!(5, new_order.seat_number);
        let item: Option<&Visitor> = order1.visitors.get(0);

        match item {
            Some(k) => {
                assert_eq!(k.order.len(), 1)
            }
            None => {
                panic!("oops not found")
            }
        }
       
    }

    #[test]

    pub fn new_visitor() {
        let mut visitor = Hotel::new();
        visitor.new_visitor(1);
        assert_eq!(1, visitor.visitors.len());
    }


}
