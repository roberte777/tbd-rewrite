use backend::Backend;
use tokio::sync::Mutex;

pub mod actions;
pub mod backend;
pub mod commands;
pub struct Terminal(pub Mutex<Option<Backend>>);
