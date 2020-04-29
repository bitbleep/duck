pub struct Vec<T>
where
    T: 'static,
{
    data: &'static mut [T],
    offset: usize,
    lock_flag: &'static core::sync::atomic::AtomicBool,
}

impl<T> Vec<T>
where
    T: Copy,
{
    pub fn new(data: &'static mut [T], lock_flag: &'static core::sync::atomic::AtomicBool) -> Self {
        Self {
            data,
            offset: 0,
            lock_flag,
        }
    }

    pub fn len(&self) -> usize {
        self.offset
    }

    pub fn capacity(&self) -> usize {
        self.data.len()
    }

    pub fn clear(&mut self) {
        self.offset = 0;
    }

    pub fn push(&mut self, value: T) {
        if self.offset == self.capacity() {
            panic!("cannot grow");
        }
        self.data[self.offset] = value;
        self.offset += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.offset {
            0 => None,
            _ => {
                self.offset -= 1;
                Some(self.data[self.offset])
            }
        }
    }

    pub fn iter(&self) -> core::slice::Iter<T> {
        self.data[..self.offset].iter()
    }

    pub fn iter_mut(&mut self) -> core::slice::IterMut<T> {
        let len = self.len();
        self.data[..len].iter_mut()
    }
}

impl<T> core::ops::Drop for Vec<T> {
    fn drop(&mut self) {
        self.lock_flag
            .store(false, core::sync::atomic::Ordering::SeqCst);
    }
}

impl<T> core::ops::Index<usize> for Vec<T>
where
    T: Copy,
{
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.offset {
            panic!("vec out of bounds");
        }
        &self.data[index]
    }
}

impl<T> core::ops::IndexMut<usize> for Vec<T>
where
    T: Copy,
{
    fn index_mut(&mut self, index: usize) -> &mut T {
        if index >= self.offset {
            panic!("vec out of bounds");
        }
        &mut self.data[index]
    }
}
