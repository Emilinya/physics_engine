use std::ops::Deref;

pub trait WrappingWindows {
    type Item;

    /// Returns an iterator over all contiguous windows of length
    /// `N`. The windows overlap, and wrap around the end of the slice
    /// such that the iterator always returns `slice.len()` items.
    ///
    /// # Examples
    ///
    /// ```
    /// let slice = &['h', 'e', 'l', 'l', 'o'];
    /// let mut iter = slice.wrapping_windows::<2>();
    /// assert_eq!(iter.next(), Some([&'h', &'e']));
    /// assert_eq!(iter.next(), Some([&'e', &'l']));
    /// assert_eq!(iter.next(), Some([&'l', &'l']));
    /// assert_eq!(iter.next(), Some([&'l', &'o']));
    /// assert_eq!(iter.next(), Some([&'o', &'h']));
    /// assert_eq!(iter.next(), None);
    /// ```
    ///
    /// If the slice is shorter than `N`:
    ///
    /// ```
    /// let slice = &['h', 'e', 'y'];
    /// let mut iter = slice.wrapping_windows::<4>();
    /// assert_eq!(iter.next(), Some([&'h', &'e', &'y', &'h']));
    /// assert_eq!(iter.next(), Some([&'e', &'y', &'h', &'e']));
    /// assert_eq!(iter.next(), Some([&'y', &'h', &'e', &'y']));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn wrapping_windows<const N: usize>(&self) -> WrappingWindowsIter<Self::Item, N>;
}

impl<T> WrappingWindows for [T] {
    type Item = T;

    fn wrapping_windows<const N: usize>(&self) -> WrappingWindowsIter<T, N> {
        WrappingWindowsIter::new(self)
    }
}

impl<T, V: Deref<Target = [T]>> WrappingWindows for V {
    type Item = T;

    fn wrapping_windows<const N: usize>(&self) -> WrappingWindowsIter<T, N> {
        self.deref().wrapping_windows()
    }
}

#[derive(Debug, Clone)]
pub struct WrappingWindowsIter<'a, T, const N: usize> {
    v: &'a [T],
    idx: usize,
    end: usize,
}

impl<'a, T, const N: usize> WrappingWindowsIter<'a, T, N> {
    fn new(v: &'a [T]) -> Self {
        Self {
            v,
            idx: 0,
            end: v.len(),
        }
    }

    fn get_array_starting_at(&self, start: usize) -> [&'a T; N] {
        let mut idx = start;
        std::array::from_fn(|_| {
            idx %= self.v.len();
            let ret = &self.v[idx];
            idx += 1;
            ret
        })
    }
}

impl<'a, T, const N: usize> Iterator for WrappingWindowsIter<'a, T, N> {
    type Item = [&'a T; N];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.end {
            None
        } else {
            let ret = self.get_array_starting_at(self.idx);
            self.idx += 1;
            Some(ret)
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.idx >= self.end {
            (0, Some(0))
        } else {
            let size = self.end - self.idx;
            (size, Some(size))
        }
    }

    #[inline]
    fn count(self) -> usize {
        self.len()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let (start, overflow) = self.idx.overflowing_add(n);
        if start >= self.end || overflow {
            self.idx = self.end;
            None
        } else {
            self.idx = start + 1;
            Some(self.get_array_starting_at(start))
        }
    }

    #[inline]
    fn last(self) -> Option<Self::Item> {
        if self.idx >= self.end {
            None
        } else {
            let start = self.end - 1;
            Some(self.get_array_starting_at(start))
        }
    }
}

impl<T, const N: usize> DoubleEndedIterator for WrappingWindowsIter<'_, T, N> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.idx >= self.end {
            None
        } else {
            let ret = self.get_array_starting_at(self.end - 1);
            self.end -= 1;
            Some(ret)
        }
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let (end, overflow) = self.end.overflowing_sub(n);
        if self.idx >= self.end || overflow {
            self.end = self.idx;
            None
        } else {
            self.end = end - 1;
            Some(self.get_array_starting_at(end - 1))
        }
    }
}

impl<T, const N: usize> ExactSizeIterator for WrappingWindowsIter<'_, T, N> {}

#[cfg(test)]
mod tests {
    use super::WrappingWindows;

    #[test]
    fn test_wrapping_windows_slice() {
        let slice = &['h', 'e', 'l', 'l', 'o'];

        let mut iter = slice.wrapping_windows::<2>();
        assert_eq!(iter.next(), Some([&'h', &'e']));
        assert_eq!(iter.next(), Some([&'e', &'l']));
        assert_eq!(iter.next(), Some([&'l', &'l']));
        assert_eq!(iter.next(), Some([&'l', &'o']));
        assert_eq!(iter.next(), Some([&'o', &'h']));
        assert_eq!(iter.next(), None);

        let mut iter = slice.wrapping_windows::<4>();
        assert_eq!(iter.next(), Some([&'h', &'e', &'l', &'l']));
        assert_eq!(iter.next(), Some([&'e', &'l', &'l', &'o']));
        assert_eq!(iter.next(), Some([&'l', &'l', &'o', &'h']));
        assert_eq!(iter.next(), Some([&'l', &'o', &'h', &'e']));
        assert_eq!(iter.next(), Some([&'o', &'h', &'e', &'l']));
        assert_eq!(iter.next(), None);

        let mut iter = slice.wrapping_windows::<3>();
        assert_eq!(iter.len(), 5);
        assert_eq!(iter.nth(1), Some([&'e', &'l', &'l']));
        assert_eq!(iter.nth(1), Some([&'l', &'o', &'h']));
        assert_eq!(iter.nth(1), None);
        assert_eq!(iter.nth(1), None);

        let iter = slice.wrapping_windows::<2>();
        assert_eq!(iter.last(), Some([&'o', &'h']));

        let slice = &['h', 'e', 'y'];

        let mut iter = slice.wrapping_windows::<4>();
        assert_eq!(iter.next(), Some([&'h', &'e', &'y', &'h']));
        assert_eq!(iter.next(), Some([&'e', &'y', &'h', &'e']));
        assert_eq!(iter.next(), Some([&'y', &'h', &'e', &'y']));
        assert_eq!(iter.next(), None);

        let numbers = &[1, 2, 3, 4, 5, 6];

        let mut iter = numbers.wrapping_windows::<2>();
        assert_eq!(iter.next(), Some([&1, &2]));
        assert_eq!(iter.next_back(), Some([&6, &1]));
        assert_eq!(iter.next_back(), Some([&5, &6]));
        assert_eq!(iter.next(), Some([&2, &3]));
        assert_eq!(iter.next(), Some([&3, &4]));
        assert_eq!(iter.next(), Some([&4, &5]));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);

        let mut iter = numbers.wrapping_windows::<3>();
        assert_eq!(iter.nth(1), Some([&2, &3, &4]));
        assert_eq!(iter.nth_back(2), Some([&4, &5, &6]));
        assert_eq!(iter.nth_back(0), Some([&3, &4, &5]));
        assert_eq!(iter.nth_back(1), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_wrapping_windows_vec() {
        let vec = vec![9, 15, 999];

        let mut iter = vec.wrapping_windows::<2>();
        assert_eq!(iter.next(), Some([&9, &15]));
        assert_eq!(iter.next(), Some([&15, &999]));
        assert_eq!(iter.next(), Some([&999, &9]));
        assert_eq!(iter.next(), None);
    }
}
