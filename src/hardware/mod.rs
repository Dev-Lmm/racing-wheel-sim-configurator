pub mod logitech;

pub trait RaceWheel {
    fn get_name(&self) -> &str;
    fn set_rotation(&self, degrees: u32) -> Result<(), std::io::Error>;
}