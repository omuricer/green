pub mod entity;
pub use entity::*;
pub mod repository;
pub use repository::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
