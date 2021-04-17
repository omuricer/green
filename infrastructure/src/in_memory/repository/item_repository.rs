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
    fn all(&self) -> HashMap<i32, Item> {
        self.map.clone()
    }
    fn find(&self, id: &i32) -> Option<&Item> {
        self.map.get(id)
    }
}
impl ItemRepositoryTrait for ItemRepository {}

#[cfg(test)]
mod tests {
    use super::*;

    fn one_model() -> Item {
        let id = 1;
        Item {
            id: id,
            name: String::from("名前"),
            name_en: String::from("name"),
        }
    }

    fn some_models() -> HashMap<i32, Item> {
        let mut models: HashMap<i32, Item> = HashMap::new();
        for id in 1..11 {
            let item = Item {
                id: id,
                name: String::from("名前"),
                name_en: String::from("name"),
            };
            models.insert(id, item);
        }
        models
    }

    #[test]
    fn repository_is_addable() {
        let model = one_model();
        let mut r = ItemRepository::new();
        r.add(model.clone());
    }

    #[test]
    fn repository_is_addable_consecutive() {
        let models = some_models();
        let mut r = ItemRepository::new();
        for model in models.values() {
            r.add(model.clone());
        }
    }

    #[test]
    fn repository_is_gettable_all() {
        let models = some_models();
        let mut r = ItemRepository::new();
        for model in models.values() {
            r.add(model.clone());
        }
        assert_eq!(models, r.all());
    }

    #[test]
    fn repository_is_findable() {
        let model = one_model();
        let mut r = ItemRepository::new();
        r.add(model.clone());
        let actual = r.find(&model.id).expect("error!");
        assert_eq!(&model, actual);
    }
}
