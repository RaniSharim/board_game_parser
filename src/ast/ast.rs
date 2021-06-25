pub enum Action {
    Draw(i32, Box<Item>),
}

pub enum Item {
    Card<String>
}