
#[derive(Debug)]
pub enum Action {
    Draw(i32, Box<Resource>, Box<ResourcePool>, FaceDirection),
    Add(i32, Box<Resource>, Box<ResourcePool>, FaceDirection),
    Shuffel(Box<ResourcePool>),
    Discard(i32, Box<Resource>, Box<ResourcePool>, FaceDirection),
    
    Chain(Vec<Action>)
}

#[derive(Debug)]
pub enum Resource {
    Card(String),
    Token(String)
}

#[derive(Debug)]

pub enum FaceDirection {
    FaceUp,
    FaceDown
}


#[derive(Debug)]
pub enum ResourcePool {
    Discard,
    Deck(String),
    Hand
}