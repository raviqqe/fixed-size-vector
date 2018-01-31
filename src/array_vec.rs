use std::mem::replace;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ArrayVec<A> {
    array: A,
    start: usize,
    length: usize,
}

impl<A> ArrayVec<A> {
    pub fn new() -> Self
    where
        A: Default,
    {
        ArrayVec {
            array: Default::default(),
            start: 0,
            length: 0,
        }
    }

    pub fn enqueue<T>(&mut self, x: T) -> bool
    where
        A: AsRef<[T]> + AsMut<[T]>,
    {
        if self.length == self.capacity() {
            return false;
        }

        let c = self.capacity();
        self.array.as_mut()[(self.start + self.length) % c] = x;
        self.length += 1;
        true
    }

    pub fn dequeue<T: Default>(&mut self) -> Option<T>
    where
        A: AsRef<[T]> + AsMut<[T]>,
    {
        if self.length == 0 {
            return None;
        }

        let x = replace(&mut self.array.as_mut()[self.start], Default::default());
        self.start = (self.start + 1) % self.capacity();
        self.length -= 1;
        Some(x)
    }

    pub fn len(&self) -> usize {
        self.length
    }

    fn capacity<T>(&self) -> usize
    where
        A: AsRef<[T]>,
    {
        self.array.as_ref().len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let _: ArrayVec<[usize; 1]> = ArrayVec::new();
        let _: ArrayVec<[usize; 2]> = ArrayVec::new();
    }

    #[test]
    fn enqueue() {
        let mut a: ArrayVec<[usize; 1]> = ArrayVec::new();

        assert_eq!(a.len(), 0);
        assert!(a.enqueue(42));
        assert_eq!(a.len(), 1);
        assert!(!a.enqueue(42));
        assert_eq!(a.len(), 1);

        let mut a: ArrayVec<[usize; 2]> = ArrayVec::new();

        assert_eq!(a.len(), 0);
        assert!(a.enqueue(42));
        assert_eq!(a.len(), 1);
        assert!(a.enqueue(42));
        assert_eq!(a.len(), 2);
        assert!(!a.enqueue(42));
        assert_eq!(a.len(), 2);
    }

    #[test]
    fn dequeue() {
        let mut a: ArrayVec<[usize; 1]> = ArrayVec::new();

        assert!(a.enqueue(42));

        assert_eq!(a.dequeue(), Some(42));
        assert_eq!(a.len(), 0);

        let mut a: ArrayVec<[usize; 2]> = ArrayVec::new();

        assert!(a.enqueue(123));
        assert!(a.enqueue(42));

        assert_eq!(a.dequeue(), Some(123));
        assert_eq!(a.len(), 1);
        assert_eq!(a.dequeue(), Some(42));
        assert_eq!(a.len(), 0);
    }

    #[test]
    fn enqueue_and_dequeue_over_boundary() {
        let mut a: ArrayVec<[usize; 2]> = ArrayVec::new();

        assert!(a.enqueue(1));
        assert!(a.enqueue(2));

        for i in 3..64 {
            assert_eq!(a.dequeue(), Some(i - 2));
            assert_eq!(a.len(), 1);
            assert!(a.enqueue(i));
            assert_eq!(a.len(), 2);
        }
    }
}
