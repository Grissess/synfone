use event;

pub trait EventSource {
    fn events<T>(&mut self) -> T
        where T: Iterator<Item=event::Event>;
}
