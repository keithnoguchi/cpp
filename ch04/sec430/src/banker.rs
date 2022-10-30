//! Dijkstra Banker Algorithm
use std::sync::{Arc, Mutex};

#[derive(Clone)]
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

impl<const R: usize, const N: usize> Resource<R, N> {
    fn new(available: [usize; R], max: [[usize; R]; N]) -> Self {
        // nothing allocated yet.
        let allocation = [[0; R]; N];
        Self {
            available,
            allocation,
            max,
        }
    }

    // take one resource identified by res_id to id.
    fn take(&mut self, id: usize, res_id: usize) -> bool {
        if id >= N
            || res_id >= R
            || self.available[res_id] == 0
            || self.allocation[id][res_id] >= self.max[id][res_id]
        {
            return false;
        }
        // grab a resource and check if it's safe,
        // otherwise, return it.
        self.available[res_id] -= 1;
        self.allocation[id][res_id] += 1;
        if self.is_safe() {
            true
        } else {
            self.allocation[id][res_id] -= 1;
            self.available[res_id] += 1;
            false
        }
    }

    // release one resource identified by res_id for thread id.
    fn release(&mut self, id: usize, res_id: usize) {
        if id >= N || res_id >= R || self.allocation[id][res_id] == 0 {
            return;
        }
        self.allocation[id][res_id] -= 1;
        self.available[res_id] += 1;
    }

    // simulation of the current allocation to prove there is no
    // possible starvation on any workers.
    fn is_safe(&self) -> bool {
        let mut finish = [false; N];
        let mut available = self.available;

        // simulate all the workers are safe to allocate
        // without any starvation.
        loop {
            let mut found = false;
            let mut num_true = 0;

            // check for each workers.
            for (id, allocated) in self.allocation.iter().enumerate() {
                if finish[id] {
                    num_true += 1;
                    continue;
                }

                // how much more resources needed for this worker.
                let need = self.max[id]
                    .iter()
                    .zip(allocated)
                    .map(|(max, allocated)| max - allocated);

                // making sure there is enough resource remains
                // for this worker to allocate reousrces without
                // the starvation in the future.
                let is_available = available
                    .iter()
                    .zip(need)
                    .all(|(available, need)| *available >= need);

                if is_available {
                    found = true;
                    finish[id] = true;
                    for (w, a) in available.iter_mut().zip(allocated) {
                        // We can return back the current allocated
                        // resource to the available pool for the
                        // remaining iteration, because the worker will
                        // return the resource once it gets the full
                        // resource in the future.
                        //
                        // This is deep and need to meditate to sink in...
                        *w += *a;
                    }
                    break;
                }
            }

            if num_true == N {
                // okay, we proved that all the workers are
                // safe from starvation.
                return true;
            }

            if !found {
                // non of the workers are safe in this simulation
                // iteration, meaning this allocation proposal
                // will lead to the resource starvation.
                //
                // It's not safe and return false.
                break;
            }
        }
        false
    }
}
