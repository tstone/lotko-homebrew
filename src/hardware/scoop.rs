use frontbox::prelude::*;

#[derive(serde::Serialize, Event)]
pub struct ScoopBallEntered(pub &'static str);

#[derive(serde::Serialize, Event)]
pub struct ScoopBallExited(pub &'static str);
