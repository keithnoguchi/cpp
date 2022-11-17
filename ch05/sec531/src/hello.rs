//! 5.3.1 Future by async/await
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct Hello {
    id: u64,
    buf: Option<String>,
    state: State,
}

enum State {
    Hello,
    World,
    End,
}

impl Future for Hello {
    type Output = String;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.state {
            State::Hello => {
                self.buf.as_mut().map(|buf| {
                    buf.push_str("Hello, ");
                    buf
                });
                self.state = State::World;
                Poll::Pending
            }
            State::World => {
                self.buf.as_mut().map(|buf| {
                    buf.push_str("World!");
                    buf
                });
                self.state = State::End;
                Poll::Pending
            }
            State::End => match self.buf.take() {
                Some(buf) => Poll::Ready(format!("{}: {buf}", self.id)),
                None => panic!("we need to implement Fuse..."),
            },
        }
    }
}

impl Hello {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            buf: Some(String::new()),
            state: State::Hello,
        }
    }
}
