pub trait ToValue {
    fn to_value(&self) -> String;
}

pub trait Start {
    fn start(self);
}
