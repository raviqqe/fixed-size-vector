use std::mem::replace;

use super::error::CapacityError;

#[derive(Clone, Copy, Debug)]
pub struct ArrayQueue<A> {
    array: A,
    start: usize,
    length: usize,
}

impl<A> ArrayQueue<A> {
    pub fn new() -> Self
    where
        A: Default,
    {
        ArrayQueue {
            array: Default::default(),
            start: 0,
            length: 0,
        }
    }

    pub fn first<T>(&self) -> Option<&T>
    where
        A: AsRef<[T]>,
    {
        self.element(0)
    }

    pub fn first_mut<T>(&mut self) -> Option<&mut T>
    where
        A: AsRef<[T]> + AsMut<[T]>,
    {
        self.element_mut(0)
    }

    pub fn last<T>(&self) -> Option<&T>
    where
        A: AsRef<[T]>,
    {
        if self.is_empty() {
            return None;
        }

        self.element(self.length - 1)
    }

    pub fn last_mut<T>(&mut self) -> Option<&mut T>
    where
        A: AsRef<[T]> + AsMut<[T]>,
    {
        if self.is_empty() {
            return None;
        }

        let i = self.length - 1;
        self.element_mut(i)
    }

    fn element<T>(&self, i: usize) -> Option<&T>
    where
        A: AsRef<[T]>,
    {
        if self.is_empty() {
            None
        } else {
            Some(&self.array.as_ref()[self.index(i)])
        }
    }

    fn element_mut<T>(&mut self, i: usize) -> Option<&mut T>
    where
        A: AsRef<[T]> + AsMut<[T]>,
    {
        if self.is_empty() {
            None
        } else {
            let i = self.index(i);
            Some(&mut self.array.as_mut()[i])
        }
    }

    pub fn push_back<T: Clone>(&mut self, x: &T) -> Result<(), CapacityError>
    where
        A: AsRef<[T]> + AsMut<[T]>,
    {
        if self.is_full() {
            return Err(CapacityError);
        }

        let i = self.index(self.length);
        self.array.as_mut()[i] = x.clone();
        self.length += 1;
        Ok(())
    }

    pub fn push_front<T: Clone>(&mut self, x: &T) -> Result<(), CapacityError>
    where
        A: AsRef<[T]> + AsMut<[T]>,
    {
        if self.is_full() {
            return Err(CapacityError);
        }

        self.start = self.index(self.capacity() - 1);
        self.array.as_mut()[self.start] = x.clone();
        self.length += 1;
        Ok(())
    }

    pub fn pop_back<T: Default>(&mut self) -> Option<T>
    where
        A: AsRef<[T]> + AsMut<[T]>,
    {
        if self.is_empty() {
            return None;
        }

        let x = replace(
            &mut self.array.as_mut()[self.length - 1],
            Default::default(),
        );
        self.length -= 1;
        Some(x)
    }

    pub fn pop_front<T: Default>(&mut self) -> Option<T>
    where
        A: AsRef<[T]> + AsMut<[T]>,
    {
        if self.is_empty() {
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

impl<A: Default> Default for ArrayQueue<A> {
    fn default() -> Self {
        ArrayQueue::new()
    }
}

impl<'a, T: 'a, A: AsRef<[T]>> IntoIterator for &'a ArrayQueue<A>
where
    &'a A: IntoIterator<Item = &'a T>,
{
    type Item = &'a T;
    type IntoIter = ArrayQueueIterator<'a, A>;

    fn into_iter(self) -> Self::IntoIter {
        ArrayQueueIterator {
            queue: self,
            current: 0,
        }
    }
}

impl<'a, T: 'a, A: AsRef<[T]> + AsMut<[T]>> IntoIterator for &'a mut ArrayQueue<A>
where
    &'a A: IntoIterator<Item = &'a T>,
{
    type Item = &'a mut T;
    type IntoIter = ArrayQueueMutIterator<'a, A>;

    fn into_iter(self) -> Self::IntoIter {
        ArrayQueueMutIterator {
            queue: self,
            current: 0,
        }
    }
}

#[derive(Debug)]
pub struct ArrayQueueIterator<'a, A: 'a> {
    queue: &'a ArrayQueue<A>,
    current: usize,
}

impl<'a, T: 'a, A: AsRef<[T]>> Iterator for ArrayQueueIterator<'a, A>
where
    &'a A: IntoIterator<Item = &'a T>,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.queue.length {
            return None;
        }

        let x = &self.queue.array.as_ref()[self.queue.index(self.current)];
        self.current += 1;
        Some(x)
    }
}

#[derive(Debug)]
pub struct ArrayQueueMutIterator<'a, A: 'a> {
    queue: &'a mut ArrayQueue<A>,
    current: usize,
}

impl<'a, T: 'a, A: AsRef<[T]> + AsMut<[T]>> Iterator for ArrayQueueMutIterator<'a, A>
where
    &'a A: IntoIterator<Item = &'a T>,
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.queue.length {
            return None;
        }

        let i = self.queue.index(self.current);
        let x = &mut self.queue.array.as_mut()[i] as *mut T;
        self.current += 1;
        Some(unsafe { &mut *x })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let _: ArrayQueue<[usize; 1]> = ArrayQueue::new();
        let _: ArrayQueue<[usize; 2]> = ArrayQueue::new();
    }

    #[test]
    fn first_and_last() {
        let mut a: ArrayQueue<[usize; 2]> = ArrayQueue::new();

        assert_eq!(a.first(), None);
        assert_eq!(a.first_mut(), None);
        assert_eq!(a.last(), None);
        assert_eq!(a.last_mut(), None);

        assert!(a.push_back(&1).is_ok());

        assert_eq!(a.first(), Some(&1));
        assert_eq!(a.first_mut(), Some(&mut 1));
        assert_eq!(a.last(), Some(&1));
        assert_eq!(a.last_mut(), Some(&mut 1));

        assert!(a.push_back(&2).is_ok());

        assert_eq!(a.first(), Some(&1));
        assert_eq!(a.first_mut(), Some(&mut 1));
        assert_eq!(a.last(), Some(&2));
        assert_eq!(a.last_mut(), Some(&mut 2));
    }

    #[test]
    fn push_back() {
        let mut a: ArrayQueue<[usize; 1]> = ArrayQueue::new();

        assert_eq!(a.len(), 0);
        assert!(a.push_back(&42).is_ok());
        assert_eq!(a.len(), 1);
        assert_eq!(a.push_back(&42), Err(CapacityError));
        assert_eq!(a.len(), 1);

        let mut a: ArrayQueue<[usize; 2]> = ArrayQueue::new();

        assert_eq!(a.len(), 0);
        assert!(a.push_back(&42).is_ok());
        assert_eq!(a.len(), 1);
        assert!(a.push_back(&42).is_ok());
        assert_eq!(a.len(), 2);
        assert_eq!(a.push_back(&42), Err(CapacityError));
        assert_eq!(a.len(), 2);
    }

    #[test]
    fn push_front() {
        let mut a: ArrayQueue<[usize; 1]> = ArrayQueue::new();

        assert_eq!(a.len(), 0);
        assert!(a.push_front(&42).is_ok());
        assert_eq!(a.len(), 1);
        assert_eq!(a.push_front(&42), Err(CapacityError));
        assert_eq!(a.len(), 1);

        let mut a: ArrayQueue<[usize; 2]> = ArrayQueue::new();

        assert_eq!(a.len(), 0);
        assert!(a.push_front(&1).is_ok());
        assert_eq!(a.first(), Some(&1));
        assert_eq!(a.last(), Some(&1));
        assert_eq!(a.len(), 1);
        assert!(a.push_front(&2).is_ok());
        assert_eq!(a.first(), Some(&2));
        assert_eq!(a.last(), Some(&1));
        assert_eq!(a.len(), 2);
        assert_eq!(a.push_front(&3), Err(CapacityError));
        assert_eq!(a.len(), 2);
    }

    #[test]
    fn pop_back() {
        let mut a: ArrayQueue<[usize; 1]> = ArrayQueue::new();

        assert!(a.push_back(&42).is_ok());

        assert_eq!(a.pop_back(), Some(42));
        assert_eq!(a.len(), 0);

        let mut a: ArrayQueue<[usize; 2]> = ArrayQueue::new();

        assert!(a.push_back(&123).is_ok());
        assert!(a.push_back(&42).is_ok());

        assert_eq!(a.pop_back(), Some(42));
        assert_eq!(a.first(), Some(&123));
        assert_eq!(a.last(), Some(&123));
        assert_eq!(a.len(), 1);
        assert_eq!(a.pop_back(), Some(123));
        assert_eq!(a.len(), 0);
    }

    #[test]
    fn pop_front() {
        let mut a: ArrayQueue<[usize; 1]> = ArrayQueue::new();

        assert!(a.push_back(&42).is_ok());

        assert_eq!(a.pop_front(), Some(42));
        assert_eq!(a.len(), 0);

        let mut a: ArrayQueue<[usize; 2]> = ArrayQueue::new();

        assert!(a.push_back(&123).is_ok());
        assert!(a.push_back(&42).is_ok());

        assert_eq!(a.pop_front(), Some(123));
        assert_eq!(a.first(), Some(&42));
        assert_eq!(a.last(), Some(&42));
        assert_eq!(a.len(), 1);
        assert_eq!(a.pop_front(), Some(42));
        assert_eq!(a.len(), 0);
    }

    #[test]
    fn push_and_pop_across_edges() {
        let mut a: ArrayQueue<[usize; 2]> = ArrayQueue::new();

        assert!(a.push_back(&1).is_ok());
        assert!(a.push_back(&2).is_ok());

        for i in 3..64 {
            assert_eq!(a.pop_front(), Some(i - 2));
            assert_eq!(a.len(), 1);
            assert!(a.push_back(&i).is_ok());
            assert_eq!(a.len(), 2);
        }
    }

    #[test]
    fn is_empty() {
        let a: ArrayQueue<[usize; 1]> = ArrayQueue::new();
        assert!(a.is_empty());

        let a: ArrayQueue<[usize; 2]> = ArrayQueue::new();
        assert!(a.is_empty());
    }

    #[test]
    fn is_full() {
        let mut a: ArrayQueue<[usize; 1]> = ArrayQueue::new();
        assert!(a.push_back(&0).is_ok());
        assert!(a.is_full());

        let mut a: ArrayQueue<[usize; 2]> = ArrayQueue::new();
        assert!(a.push_back(&0).is_ok());
        assert!(a.push_back(&0).is_ok());
        assert!(a.is_full());
    }

    #[test]
    fn iterator() {
        let mut a: ArrayQueue<[usize; 2]> = ArrayQueue::new();

        assert!(a.push_back(&0).is_ok());
        assert!(a.push_back(&1).is_ok());

        for (i, e) in a.into_iter().enumerate() {
            assert_eq!(*e, i);
        }
    }

    #[test]
    fn iterator_across_edges() {
        let mut a: ArrayQueue<[usize; 2]> = ArrayQueue::new();

        assert!(a.push_back(&42).is_ok());
        a.pop_front();
        assert!(a.push_back(&0).is_ok());
        assert!(a.push_back(&1).is_ok());

        for (i, e) in a.into_iter().enumerate() {
            assert_eq!(*e, i);
        }
    }

    #[test]
    fn iterator_mut() {
        let mut a: ArrayQueue<[usize; 2]> = ArrayQueue::new();

        assert!(a.push_back(&0).is_ok());
        assert!(a.push_back(&1).is_ok());

        for (i, e) in (&mut a).into_iter().enumerate() {
            assert_eq!(*e, i);
            *e = 42;
        }
    }

    #[test]
    fn reference_elements() {
        let mut a: ArrayQueue<[Box<usize>; 2]> = ArrayQueue::new();
        assert!(a.push_back(&Box::new(42)).is_ok());
    }
}
