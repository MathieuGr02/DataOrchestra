use std::thread::JoinHandle;

pub trait ToValue {
    fn to_value(&self) -> String;
}

pub trait Start<T> {
    fn start(self) -> JoinHandle<T>;
}
