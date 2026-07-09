use std::any::Any;
use uuid::Uuid;

pub trait Observable {
    fn get_signature(&self) -> String;
    fn as_any(&self) -> &dyn Any;
}

pub trait AutoObservable: Observable {
    fn observe(&self, observation_id: Uuid) -> std::io::Result<()>;
}
