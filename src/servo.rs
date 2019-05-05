use futures::{Async, Future, Poll};

const DEFAULT_VALUE: u8 = 128;

pub struct Servo {
    value: u8,
}

impl Servo {
    pub fn new(value: u8) -> Self {
        Self { value }
    }

    pub fn update(&mut self) {}

    pub fn set_value(&mut self, value: u8) {
        self.value = value;
    }
}

impl Default for Servo {
    fn default() -> Self {
        Self {
            value: DEFAULT_VALUE,
        }
    }
}

impl Future for Servo {
    type Item = Value;
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        Ok(Async::NotReady)
    }
}

pub struct Value(u8);
