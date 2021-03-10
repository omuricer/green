// use domain::entity::Item;
// use domain::repository::Repository as RepositoryTrait;
// use std::collections::HashMap;

// pub struct Repository<T> {
//     map: HashMap<i32, T>,
// }

// impl<T> RepositoryTrait<T> for Repository<T> {
//     fn new() -> Self {
//         Repository {
//             map: HashMap::new(),
//         }
//     }
//     fn add(&mut self, model: T) {
//         self.map.insert(1, model);
//     }
//     fn find(&self, id: &i32) -> Option<&T> {
//         self.map.get(id)
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn repository_is_addable_and_findable() {
//         let id = 1;
//         let model = Item {
//             id: id,
//             name: String::from("名前"),
//             name_en: String::from("name"),
//         };
//         let mut r = ItemRepository::new();
//         r.add(model.clone());
//         let item = r.find(&id).expect("error!");
//         assert_eq!(model, *item);
//     }
// }
