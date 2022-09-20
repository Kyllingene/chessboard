#[derive(Debug)]
pub struct ClockSettings {
    pub limit: i32,
    pub increment: i32,

    pub is_correspondence: bool,
    pub days: i32,
}