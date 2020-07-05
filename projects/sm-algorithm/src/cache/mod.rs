use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

mod traits;

#[derive(Debug)]
pub struct GlobalCache {
    /// what's the hell?
    inner: Arc<Mutex<HashMap<String, Arc<Mutex<Cache>>>>>,
}

#[derive(Debug)]
pub enum Cache {}
