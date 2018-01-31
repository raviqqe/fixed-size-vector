use std::mem::replace;

use super::error::CapacityError;

#[derive(Clone, Copy, Debug)]
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

    pub fn push<T: Clone>(&mut self, x: &T) -> Result<(), CapacityError>
    where
        A: AsRef<[T]> + AsMut<[T]>,
    {
        if self.length == self.capacity() {
            return Err(CapacityError);
        }

        let i = self.index(self.length);
        self.array.as_mut()[i] = x.clone();
        self.length += 1;
        Ok(())
    }

    pub fn pop_front<T: Default>(&mut self) -> Option<T>
    where
        A: AsRef<[T]> + AsMut<[T]>,
    {
        if self.length == 0 {
            return None;
        }

        let x = replace(&mut self.array.as_mut()[self.start], Default::default());
        self.start = self.index(1);
        self.length -= 1;
        Some(x)
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty<T>(&self) -> bool
    where
        A: AsRef<[T]>,
    {
        self.len() == 0
    }

    pub fn is_full<T>(&self) -> bool
    where
        A: AsRef<[T]>,
    {
        self.len() == self.capacity()
    }

    fn index<T>(&self, i: usize) -> usize
    where
        A: AsRef<[T]>,
    {
        (self.start + i) % self.capacity()
    }

    fn capacity<T>(&self) -> usize
    where
        A: AsRef<[T]>,
    {
        self.array.as_ref().len()
    }
}

impl<A: Default> Default for ArrayVec<A> {
    fn default() -> Self {
        ArrayVec::new()
    }
}

impl<'a, T: 'a, A: AsRef<[T]>> IntoIterator for &'a ArrayVec<A>
where
    &'a A: IntoIterator<Item = &'a T>,
{
    type Item = &'a T;
    type IntoIter = ArrayVecIterator<'a, A>;

    fn into_iter(self) -> Self::IntoIter {
        ArrayVecIterator {
            vec: self,
            current: 0,
        }
    }
}

impl<'a, T: 'a, A: AsRef<[T]> + AsMut<[T]>> IntoIterator for &'a mut ArrayVec<A>
where
    &'a A: IntoIterator<Item = &'a T>,
{
    type Item = &'a mut T;
    type IntoIter = ArrayVecMutIterator<'a, A>;

    fn into_iter(self) -> Self::IntoIter {
        ArrayVecMutIterator {
            vec: self,
            current: 0,
        }
    }
}

#[derive(Debug)]
pub struct ArrayVecIterator<'a, A: 'a> {
    vec: &'a ArrayVec<A>,
    current: usize,
}

impl<'a, T: 'a, A: AsRef<[T]>> Iterator for ArrayVecIterator<'a, A>
where
    &'a A: IntoIterator<Item = &'a T>,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.vec.length {
            return None;
        }

        let x = &self.vec.array.as_ref()[self.vec.index(self.current)];
        self.current += 1;
        Some(x)
    }
}

#[derive(Debug)]
pub struct ArrayVecMutIterator<'a, A: 'a> {
    vec: &'a mut ArrayVec<A>,
    current: usize,
}

impl<'a, T: 'a, A: AsRef<[T]> + AsMut<[T]>> Iterator for ArrayVecMutIterator<'a, A>
where
    &'a A: IntoIterator<Item = &'a T>,
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.vec.length {
            return None;
        }

        let i = self.vec.index(self.current);
        let x = &mut self.vec.array.as_mut()[i] as *mut T;
        self.current += 1;
        Some(unsafe { &mut *x })
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
    fn push() {
        let mut a: ArrayVec<[usize; 1]> = ArrayVec::new();

        assert_eq!(a.len(), 0);
        assert!(a.push(&42).is_ok());
        assert_eq!(a.len(), 1);
        assert_eq!(a.push(&42), Err(CapacityError));
        assert_eq!(a.len(), 1);

        let mut a: ArrayVec<[usize; 2]> = ArrayVec::new();

        assert_eq!(a.len(), 0);
        assert!(a.push(&42).is_ok());
        assert_eq!(a.len(), 1);
        assert!(a.push(&42).is_ok());
        assert_eq!(a.len(), 2);
        assert_eq!(a.push(&42), Err(CapacityError));
        assert_eq!(a.len(), 2);
    }

    #[test]
    fn pop_front() {
        let mut a: ArrayVec<[usize; 1]> = ArrayVec::new();

        assert!(a.push(&42).is_ok());

        assert_eq!(a.pop_front(), Some(42));
        assert_eq!(a.len(), 0);

        let mut a: ArrayVec<[usize; 2]> = ArrayVec::new();

        assert!(a.push(&123).is_ok());
        assert!(a.push(&42).is_ok());

        assert_eq!(a.pop_front(), Some(123));
        assert_eq!(a.len(), 1);
        assert_eq!(a.pop_front(), Some(42));
        assert_eq!(a.len(), 0);
    }

    #[test]
    fn push_and_pop_front_across_edges() {
        let mut a: ArrayVec<[usize; 2]> = ArrayVec::new();

        assert!(a.push(&1).is_ok());
        assert!(a.push(&2).is_ok());

        for i in 3..64 {
            assert_eq!(a.pop_front(), Some(i - 2));
            assert_eq!(a.len(), 1);
            assert!(a.push(&i).is_ok());
            assert_eq!(a.len(), 2);
        }
    }

    #[test]
    fn is_empty() {
        let a: ArrayVec<[usize; 1]> = ArrayVec::new();
        assert!(a.is_empty());

        let a: ArrayVec<[usize; 2]> = ArrayVec::new();
        assert!(a.is_empty());
    }

    #[test]
    fn is_full() {
        let mut a: ArrayVec<[usize; 1]> = ArrayVec::new();
        assert!(a.push(&0).is_ok());
        assert!(a.is_full());

        let mut a: ArrayVec<[usize; 2]> = ArrayVec::new();
        assert!(a.push(&0).is_ok());
        assert!(a.push(&0).is_ok());
        assert!(a.is_full());
    }

    #[test]
    fn iterator() {
        let mut a: ArrayVec<[usize; 2]> = ArrayVec::new();

        assert!(a.push(&0).is_ok());
        assert!(a.push(&1).is_ok());

        for (i, e) in a.into_iter().enumerate() {
            assert_eq!(*e, i);
        }
    }

    #[test]
    fn iterator_across_edges() {
        let mut a: ArrayVec<[usize; 2]> = ArrayVec::new();

        assert!(a.push(&42).is_ok());
        a.pop_front();
        assert!(a.push(&0).is_ok());
        assert!(a.push(&1).is_ok());

        for (i, e) in a.into_iter().enumerate() {
            assert_eq!(*e, i);
        }
    }

    #[test]
    fn iterator_mut() {
        let mut a: ArrayVec<[usize; 2]> = ArrayVec::new();

        assert!(a.push(&0).is_ok());
        assert!(a.push(&1).is_ok());

        for (i, e) in (&mut a).into_iter().enumerate() {
            assert_eq!(*e, i);
            *e = 42;
        }
    }
}
