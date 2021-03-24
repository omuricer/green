use std::collections::HashMap;

pub trait Repository<T> {
    fn new() -> Self;
    fn add(&mut self, model: T);
    fn all(&self) -> HashMap<i32, T>;
    fn find(&self, id: &i32) -> Option<&T>;
}
