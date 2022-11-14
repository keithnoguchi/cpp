//! 5.2.1 Coroutine/Task with Future Trait
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct Hello {
    id: u64,
    buf: String,
    state: State,
}

enum State {
    Hello,
    World,
    End,
}

impl Hello {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            buf: String::new(),
            state: State::Hello,
        }
    }
}

impl Future for Hello {
    type Output = (u64, String);

    fn poll(mut self: Pin<&mut Self>, _ctx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.state {
            State::Hello => {
                self.buf.push_str("Hello, ");
                self.state = State::World;
                Poll::Pending
            }
            State::World => {
                self.buf.push_str("World!");
                self.state = State::End;
                Poll::Pending
            }
            State::End => Poll::Ready((self.id, self.buf.clone())),
        }
    }
}
