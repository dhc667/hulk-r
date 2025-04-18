pub trait IContext {
    fn is_defined(&self, var_name: &str) -> bool;
    fn define(&mut self, var_name: &str) -> bool;
    fn push_frame(&mut self);
    fn pop_frame(&mut self);
}
