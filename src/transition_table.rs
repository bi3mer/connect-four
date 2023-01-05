#[derive(Copy, Clone)]
struct Entry {
    key: u64,
    val: i8
}

impl Entry {
    pub fn new() -> Self {
        Entry { key: 0, val: 0 }
    }
}

const TABLE_MAX_SIZE: usize = 10411033; // large prime numbers

pub struct TransitionTable {
    table: Vec<Entry>
}

impl TransitionTable {
    pub fn new() -> Self {
        TransitionTable { table: vec![Entry::new(); TABLE_MAX_SIZE] }
    }

    pub fn reset(&mut self) {
        self.table.iter_mut().for_each(|e| {
            e.key = 0;
            e.val = 0;
        });
    }

    fn index(&self, key: u64) ->  usize {
        (key as usize % self.table.len()) as usize
    }

    pub fn get(&self, key: u64) -> Option<i8> {
        let i = self.index(key);
        if self.table[i].key == key {
            Some(self.table[i].val)
        } else {
            None
        }
    }

    pub fn set(&mut self, key: u64, val: i8) {
        let i = self.index(key);
        self.table[i].key = key;
        self.table[i].val = val;
    }
}