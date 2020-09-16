use photo_core::models::User;

pub trait Profile {
    fn new_user(&self) -> User;
}

pub trait Code {
    fn code(&self) -> String;
}
