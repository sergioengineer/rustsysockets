#[derive(Clone, PartialEq, Eq)]
pub enum Role{
    Admin(String),
    User(String),
    Anonymous
}