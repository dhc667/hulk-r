pub trait IContext {
    fn is_defined(&self, var_name: str) -> bool;
    fn define(&self, var_name: str) -> bool;
    fn push_frame(&self);
    fn pop_frame(&self);
}
