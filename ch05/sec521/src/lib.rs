//! 5.2.1 Coroutine with Future Trait
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct Hello {
    id: u64,
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
            state: State::Hello,
        }
    }
}

impl Future for Hello {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _ctx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.state {
            State::Hello => {
                print!("{}: Hello, ", self.id);
                self.state = State::World;
                Poll::Pending
            }
            State::World => {
                println!("{}: World!", self.id);
                self.state = State::End;
                Poll::Pending
            }
            State::End => Poll::Ready(()),
        }
    }
}
