#[derive(Debug)]
pub struct Account {
    pub client_id: u16,
    pub available: f32,
    pub held: f32,
    pub total: f32,
    pub locked: bool,
}
