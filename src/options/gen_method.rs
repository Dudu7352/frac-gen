pub enum GenMethod {
    SinglethreadAsync,
    MultithreadAsync { threads: usize },
}