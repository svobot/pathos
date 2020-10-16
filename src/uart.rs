use core::fmt::{Result, Write};

pub struct Uart {
    base_address: usize,
}

impl Uart {
    pub fn new(base_address: usize) -> Self {
        Uart { base_address }
    }

    pub fn init(&self) {
        let ptr = self.base_address as *mut u8;
        unsafe {
            // Set word length to 8 bits in LCR
            let lcr = (1 << 0) | (1 << 1);
            ptr.add(3).write_volatile(lcr);

            // Enable FIFO in FCR
            ptr.add(2).write_volatile(1 << 0);

            // Enable receiver buffer interrupts in IER
            ptr.add(1).write_volatile(1 << 0);

            // Set the clock rate divisor as:
            // ceil ((global clock frequency) / ((signals per second) * 16)) =
            // ceil ( 22_729_000              / ( 2_400               * 16))
            let divisor: u16 = 592;
            let divisor_least = (divisor & 0xff) as u8;
            let divisor_most = (divisor >> 8) as u8;

            // Set the Divisor Latch Access Bit (DLAB) in LCR
            ptr.add(3).write_volatile(lcr | (1 << 7));

            // Write divisor
            ptr.add(0).write_volatile(divisor_least);
            ptr.add(1).write_volatile(divisor_most);

            // Clear DLAB in LCR
            ptr.add(3).write_volatile(lcr);
        }
    }

    pub fn get(&self) -> Option<u8> {
        let ptr = self.base_address as *mut u8;
        unsafe {
            if ptr.add(5).read_volatile() & 1 == 0 {
                None
            } else {
                Some(ptr.add(0).read_volatile())
            }
        }
    }

    pub fn put(&self, c: u8) {
        let ptr = self.base_address as *mut u8;
        unsafe {
            ptr.add(0).write_volatile(c);
        }
    }
}

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> Result {
        for c in s.bytes() {
            self.put(c);
        }
        Ok(())
    }
}
