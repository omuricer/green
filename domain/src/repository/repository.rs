pub trait Repository<T> {
    fn new() -> Self;
    fn add(&mut self, model: T);
    fn find(&self, id: &i32) -> Option<&T>;
}
