pub struct Item<'a> {
    // TODO: getterを定義する
    pub id: i32,
    pub name: String,
    pub name_en: String,
    pub parent: Option<&'a Item<'a>>,
    pub children: Option<&'a Item<'a>>,
}
