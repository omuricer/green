use std::collections::HashMap;

pub trait Repository<T> {
    fn new() -> Self;
    // TODO: エラー設計をして独自のエラー型を返却するようにする
    // 参考 https://qiita.com/fujitayy/items/cafe661415b6aa33d884
    // 参考 https://11takanori.medium.com/rust%E3%81%AE%E3%82%A8%E3%83%A9%E3%83%BC%E3%83%8F%E3%83%B3%E3%83%89%E3%83%AA%E3%83%B3%E3%82%B0-6660cd4d16c0
    fn add(&mut self, model: T) -> Result<(), String>;
    fn all(&self) -> HashMap<i32, T>;
    fn find(&self, id: &i32) -> Option<&T>;
}
