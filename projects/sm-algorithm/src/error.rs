pub enum Error {
    OverFlow,
    ComplexInfinity
}

pub type Out<T> = Result<T,Error>;