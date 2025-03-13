
pub trait HasPassword {
    fn has_password(&self) -> bool;
    fn is_password(&self, clear_password: &str) -> bool;
    fn set_password(&mut self, clear_password: &str);
}
