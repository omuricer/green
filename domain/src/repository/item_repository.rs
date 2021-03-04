use crate::entity::Item;

pub trait ItemRepository {
    fn new() -> Self;
    fn add(&self, model: &Item);
    fn find(&self, id: &i32) -> Item;
}
