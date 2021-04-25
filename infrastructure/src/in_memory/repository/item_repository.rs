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
    fn add(&mut self, model: Item) -> Result<(), String> {
        if self.map.contains_key(&model.id) {
            return Err(String::from("Duplicate ID!"));
        }
        self.map.insert(model.id, model);
        return Ok(());
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
    fn addable() {
        let mut r = ItemRepository::new();
        let actual = r.add(one_model().clone());
        assert_eq!(Ok(()), actual);
    }
    #[test]
    fn addable2() {
        let mut r = ItemRepository::new();
        let mut actual = r.add(one_model().clone());
        assert_eq!(Ok(()), actual);
        actual = r.add(one_model().clone());
        assert_eq!(Err(String::from("Duplicate ID!")), actual);
    }

    #[test]
    fn addable_consecutive() {
        let mut r = ItemRepository::new();
        for model in some_models().values() {
            if let Err(err) = r.add(model.clone()) {
                panic!("{}", err);
            };
        }
    }
    #[test]
    fn gettable_all() {
        let models = some_models();
        let mut r = ItemRepository::new();
        for model in models.values() {
            if let Err(err) = r.add(model.clone()) {
                panic!("{}", err);
            };
        }
        assert_eq!(models, r.all());
    }
    #[test]
    fn all_returns_empty_hashmap_when_repository_not_exist() {
        let expect: HashMap<i32, Item> = HashMap::new();

        let r = ItemRepository::new();
        let actual = r.all();
        assert_eq!(expect, actual);
    }
    #[test]
    fn findable() {
        let model = one_model();
        let mut r = ItemRepository::new();
        if let Err(err) = r.add(model.clone()) {
            panic!("{}", err);
        };
        let actual = r.find(&model.id).expect("error!");
        assert_eq!(&model, actual);
    }
    #[test]
    fn find_returns_none_when_id_not_exist() {
        let none_id = 999999;
        let r = ItemRepository::new();
        let actual = r.find(&none_id);
        assert_eq!(None, actual);
    }
}
