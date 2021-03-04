pub struct Item<'a> {
    name: String,
    name_en: String,
    parent: &'a Item<'a>,
    children: &'a Item<'a>,
}
