use super::repository::Repository;
use crate::entity::Item;

pub trait ItemRepository: Repository<Item> {
    // fn new() -> Self;
    // fn add(&mut self, model: Item);
    // fn find(&self, id: &i32) -> Option<&Item>;
}
