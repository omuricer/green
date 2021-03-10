#[derive(Debug, PartialEq, Clone)]
pub struct Item {
    // TODO: getterを定義する
    pub id: i32,
    pub name: String,
    pub name_en: String,
    // TODO: リソース効率の良いTree構造？List構造にする
    // pub parent: Option<&'a Item<'a>>,
    // pub children: Option<&'a Item<'a>>,
}
