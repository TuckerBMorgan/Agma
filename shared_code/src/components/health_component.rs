use bincode::{Decode, Encode};

#[derive(PartialEq, Debug, Copy, Clone, Encode, Decode)]
pub struct HealthComponent {
    pub current_amount: usize
}

impl HealthComponent {
    pub fn new(current_amount: usize) -> HealthComponent {
        HealthComponent {
            current_amount
        }
    }

    pub fn do_damage(&mut self, amount: usize) {
        if amount > self.current_amount {
            self.current_amount = 0;
        }
        else {
            self.current_amount -= amount;
        }
    }
}
