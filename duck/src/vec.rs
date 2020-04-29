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
    #[inline(always)]
    pub fn new(data: &'static mut [T], lock_flag: &'static core::sync::atomic::AtomicBool) -> Self {
        Self {
            data,
            offset: 0,
            lock_flag,
        }
    }

    #[inline(always)]
    pub fn capacity(&self) -> usize {
        self.data.len()
    }

    // pub fn truncate(&mut self, len: usize)

    #[inline(always)]
    pub fn as_slice(&self) -> &[T] {
        &self.data[..self.offset]
    }

    #[inline(always)]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data[..self.offset]
    }

    #[inline(always)]
    pub fn as_ptr(&self) -> *const T {
        self.data.as_ptr()
    }

    #[inline(always)]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.data.as_mut_ptr()
    }

    // pub fn swap_remove(&mut self, index: usize) -> T

    pub fn insert(&mut self, index: usize, element: T) {
        if self.len() == self.capacity() {
            panic!("cannot grow");
        }
        if index >= self.capacity() {
            panic!("out of bounds");
        }
        self.data.copy_within(index..self.offset, index + 1);
        self.data[index] = element;
        self.offset += 1;
    }

    pub fn remove(&mut self, index: usize) -> T {
        if index >= self.len() {
            panic!("out of bounds");
        }
        let element = self.data[index];
        self.data.copy_within(index + 1..self.offset, index);
        self.offset -= 1;
        element
    }

    pub fn push(&mut self, value: T) {
        if self.len() == self.capacity() {
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

    pub fn append(&mut self, other: &mut Vec<T>) {
        let space = self.capacity() - self.len();
        if space < other.len() {
            panic!("cannot grow");
        }
        self.data[self.offset..self.offset + other.len()].copy_from_slice(other.as_slice());
        self.offset += other.len();
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.offset = 0;
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.offset
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.offset == 0
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
