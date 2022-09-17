const PAGESIZE: u32 = 0x512;

pub struct Memory {
    pages: Map<i32, [i32, PAGESIZE]>,
}

impl Memory {
    pub fn read(&self, address: u64) {
        bucket = floor(address / PAGESIZE);
        page = self.pages.get(bucket);
        page[address % PAGESIZE];
    }
}

fn main() {
    /// core routine
    switch (program_counter[i]) {
        // fetch and decode
    }
}

