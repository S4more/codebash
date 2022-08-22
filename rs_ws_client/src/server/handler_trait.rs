pub trait HandleMessage<T> {
    fn handle(arg: T);
}
