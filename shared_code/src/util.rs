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