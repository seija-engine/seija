pub struct Atom<T> {
    inner:T,
    last_set:u64
}

impl<T> Atom<T> {
    pub fn new(value:T) -> Self {
        Atom { inner: value, last_set: 0 }
    }

    pub fn set(&mut self,value:T) {
        self.inner = value;
        self.last_set += 1
    }

    pub fn inner_ref(&self) -> &T { &self.inner }

    pub fn version(&self) -> u64 { self.last_set }
}

