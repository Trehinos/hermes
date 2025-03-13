use crate::http::authentication::password::HasPassword;

pub trait HasIdentity {
    fn identity(&self) -> String;
}

pub mod password;

pub trait IdentityPassword: HasIdentity + HasPassword {}

pub trait Authenticator<T: IdentityPassword> {
    fn current(&self) -> Option<T>;
    fn authenticate(&self, identity: T);
    fn is_authenticated(&self) -> bool;
    fn quash(&mut self);
}

pub trait Provider<T: IdentityPassword> {
    fn get(&self, identity: &str) -> Option<T>;
}

pub struct MultipleProvider<T: IdentityPassword> {
    providers: Vec<Box<dyn Provider<T>>>,
}

impl<T: IdentityPassword> MultipleProvider<T> {
    pub fn new(providers: Vec<Box<dyn Provider<T>>>) -> Self {
        Self { providers }
    }
}

impl<T: IdentityPassword> Provider<T> for MultipleProvider<T> {
    fn get(&self, identity: &str) -> Option<T> {
        for provider in self.providers.iter() {
            if let Some(user) = provider.get(identity) {
                return Some(user);
            }
        }
        None
    }
}
