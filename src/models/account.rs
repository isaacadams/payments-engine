use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Account {
    pub client: u16,
    pub available: f32,
    pub held: f32,
    pub total: f32,
    pub locked: bool,
}
