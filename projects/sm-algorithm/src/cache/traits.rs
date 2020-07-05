use crate::{GlobalCache, Output};

impl GlobalCache {
    pub fn gc(&mut self) -> Output {
        println!("{:?}", self.inner);
        unimplemented!()
    }
    pub fn dump(&mut self) -> Output {
        println!("{:?}", self.inner);
        unimplemented!()
    }
    pub fn load(&mut self) -> Output {
        println!("{:?}", self.inner);
        unimplemented!()
    }
}
