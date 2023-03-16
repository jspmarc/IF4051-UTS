use std::sync::{Mutex, Arc};

#[derive(Default, Clone)]
pub struct State {
    pub counter: Arc<Mutex<i32>>
}

