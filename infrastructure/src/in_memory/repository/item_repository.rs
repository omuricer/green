use domain::entity::Item;
use domain::repository::ItemRepository as ItemRepositoryTrait;

pub struct ItemRepository {}

impl ItemRepositoryTrait for ItemRepository {
    fn new() -> Self {
        ItemRepository {}
    }
    fn add(&self, model: &Item) {
        println!("Item is {:?}", model.id);
    }
    fn find(&self, id: &i32) -> Item {
        Item {
            id: *id,
            name: String::from("名前"),
            name_en: String::from("name"),
            children: None,
            parent: None,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
