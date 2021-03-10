use domain::entity::Item;
use domain::repository::ItemRepository as ItemRepositoryTrait;
use domain::repository::Repository as RepositoryTrait;
use std::collections::HashMap;

pub struct ItemRepository {
    map: HashMap<i32, Item>,
}
impl RepositoryTrait<Item> for ItemRepository {
    fn new() -> Self {
        ItemRepository {
            map: HashMap::new(),
        }
    }
    fn add(&mut self, model: Item) {
        self.map.insert(model.id, model);
    }
    fn find(&self, id: &i32) -> Option<&Item> {
        self.map.get(id)
    }
}
impl ItemRepositoryTrait for ItemRepository {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn repository_is_addable_and_findable() {
        let id = 1;
        let model = Item {
            id: id,
            name: String::from("名前"),
            name_en: String::from("name"),
        };
        let mut r = ItemRepository::new();
        r.add(model.clone());
        let item = r.find(&id).expect("error!");
        assert_eq!(model, *item);
    }
}
