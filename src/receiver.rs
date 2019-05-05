use futures::{Async, Poll, Stream};

pub struct Receiver {}

impl Receiver {
    pub fn new() -> Self {
        Self {}
    }
}

impl Stream for Receiver {
    type Item = u8;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        Ok(Async::NotReady)
    }
}
