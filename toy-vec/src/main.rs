pub struct ToyVec<T> {
    elements: Box<[T]>,
    len: usize,
}

impl<T: Default> ToyVec<T> {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            elements: Self::allocate_in_heap(capacity),
            len: 0,
        }
    }

    fn allocate_in_heap(size: usize) -> Box<[T]> {
        std::iter::repeat_with(Default::default)
            .take(size)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.elements.len()
    }

    pub fn push(&mut self, element: T) {
        if self.len == self.capacity() {
            self.grow();
        }

        self.elements[self.len] = element;
        self.len += 1;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            Some(&self.elements[index])
        } else {
            None
        }
    }

    fn grow(&self) {}
}

fn main() {
    let mut v = ToyVec::<usize>::with_capacity(10);
    v.push(1);
    v.push(2);
    if let Some(value) = v.get(1) {
        println!("{}", *value);
    }
    if let Some(value) = v.get(100) {
        println!("{}", *value);
    } else {
        println!("None");
    }
    let e = v.get(0);
    println!("{}", v.len());
    assert_eq!(e, Some(&1));
    v.push(3);
    assert_eq!(e, Some(&1));
}
