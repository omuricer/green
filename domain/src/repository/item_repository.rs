use super::repository::Repository;
use crate::entity::Item;

pub trait ItemRepository: Repository<Item> {}
