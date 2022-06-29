const RING_BUFFER_INTERNAL_STORAGE_SLOTS : usize = 32;

pub struct RingBuffer<T> {
    pub storage: Vec<T>,
    pub next_open_slot: usize
}

impl<T> RingBuffer<T> {
    pub fn new() -> RingBuffer<T> {
        RingBuffer {
            storage: vec![],
            next_open_slot: 0
        }
    }
    pub fn add_new_data(&mut self, tuple: T) {
        if self.storage.len() < RING_BUFFER_INTERNAL_STORAGE_SLOTS {
            self.storage.push(tuple);
            self.next_open_slot += 1;
            return;
        }
        let index = self.next_open_slot % self.storage.len();
        self.storage[index] = tuple;
        self.next_open_slot += 1;
    }
}

//This just running the Bresenham line drawing Algo 1 step foward
//Hopefully should make moving in a diagonal smoother
pub fn path_find(start_x: i64, start_y: i64, end_x: i64, end_y: i64) -> (i64, i64) {
    let x_direction = end_x - start_x;
    let mut y_direction = end_y - start_y;
    let mut y_update_amount = 1;

    if y_direction < 0 {
        y_update_amount = -1;
        y_direction = -y_direction;
    }

    let rename = (2 * y_direction) - x_direction;    
    if rename > 0 {
        return (start_x + x_direction.signum(), start_y + y_update_amount);
    }
    else {
        return (start_x + x_direction.signum(), start_y);
    }
}
