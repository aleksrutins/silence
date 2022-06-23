use druid::{Data, Lens};

use crate::transport::Transport;

#[derive(Clone, Data)]
pub struct Project {
    pub name: String,
}

#[derive(Clone, Data, Lens)]
pub struct GlobalState {
    pub project: Option<Project>,
    pub transport: Transport,
}

impl Default for GlobalState {
    fn default() -> Self {
        Self {
            project: None,
            transport: Transport::new(),
        }
    }
}
