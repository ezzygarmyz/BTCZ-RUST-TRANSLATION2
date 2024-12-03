/// A utility for reverse iteration over any collection implementing IntoIterator
pub struct ReverseIterator<I>
where
    I: IntoIterator,
{
    inner: I::IntoIter,
}

impl<I> ReverseIterator<I>
where
    I: IntoIterator,
{
    /// Creates a new ReverseIterator
    pub fn new(collection: I) -> Self {
        ReverseIterator {
            inner: collection.into_iter(),
        }
    }
}

impl<I> Iterator for ReverseIterator<I>
where
    I: IntoIterator,
    I::IntoIter: DoubleEndedIterator, // Ensures the iterator supports reverse iteration
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_iterator() {
        let collection = vec![1, 2, 3, 4, 5];
        let reverse_iter = ReverseIterator::new(collection);

        let result: Vec<_> = reverse_iter.collect();
        assert_eq!(result, vec![5, 4, 3, 2, 1]);
    }
}
