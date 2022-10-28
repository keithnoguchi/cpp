//! Dijkstra Banker Algorithm
use std::sync::{Arc, Mutex};

pub struct Banker<const NR_RESOURCES: usize, const NR_CLIENTS: usize> {
    state: Arc<Mutex<Resource<NR_RESOURCES, NR_CLIENTS>>>,
}

// This is yet another great example of the internal mutability.
//
// All the public functions are immutable, but internally it
// changes the state.
impl<const R: usize, const C: usize> Banker<R, C> {
    pub fn new(resource: [usize; R], max: [[usize; R]; C]) -> Self {
        let state = Resource::<R, C>::new(resource, max);
        Self {
            state: Arc::new(Mutex::new(state)),
        }
    }

    pub fn take(&self, id: usize, resource: usize) -> bool {
        let mut state = self.state.lock().unwrap();
        state.take(id, resource)
    }

    pub fn release(&self, id: usize, resource: usize) {
        let mut state = self.state.lock().unwrap();
        state.release(id, resource)
    }
}

// The Resource type is the actual implementation of the
// Dijkstra Banker Algorithm.
struct Resource<const NR_RESOURCES: usize, const NR_CLIENTS: usize> {
    // the current available resource to release.
    available: [usize; NR_RESOURCES],
    // The current allocation of the resources for each client.
    allocation: [[usize; NR_RESOURCES]; NR_CLIENTS],
    // The maximum resources needed by each client.
    max: [[usize; NR_RESOURCES]; NR_CLIENTS],
}

impl<const R: usize, const C: usize> Resource<R, C> {
    fn new(available: [usize; R], max: [[usize; R]; C]) -> Self {
        // nothing allocated yet.
        let allocation = [[0; R]; C];
        Self {
            available,
            allocation,
            max,
        }
    }

    fn take(&mut self, _id: usize, _resource: usize) -> bool {
        todo!()
    }

    fn release(&mut self, _id: usize, _resource: usize) {
        todo!()
    }

    fn is_safe(&self) -> bool {
        todo!()
    }
}
