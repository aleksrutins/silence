use druid::Data;

use crate::timeline::Timeline;

#[derive(Clone, Data)]
pub struct Project {
    pub name: String,
}

#[derive(Clone, Data)]
pub struct GlobalState {
    pub project: Option<Project>,
    pub timeline: Timeline
}

impl Default for GlobalState {
    fn default() -> Self {
        Self {
            project: None,
            timeline: Timeline::new()
        }
    }
}