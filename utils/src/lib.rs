pub trait ExtendedIter: Iterator + Sized {
    fn pop(&mut self) -> Self::Item {
        self.next().unwrap()
    }

    fn pop_back(&mut self) -> Self::Item
    where
        Self: DoubleEndedIterator,
    {
        self.next_back().unwrap()
    }

    fn sort(self) -> std::vec::IntoIter<<Self as Iterator>::Item>
    where
        Self::Item: Ord,
    {
        let mut vec: Vec<_> = self.collect();
        vec.sort();
        vec.into_iter()
    }

    fn sort_by_key<K, F>(self, f: F) -> std::vec::IntoIter<<Self as Iterator>::Item>
    where
        F: FnMut(&Self::Item) -> K,
        K: Ord,
    {
        let mut vec: Vec<_> = self.collect();
        vec.sort_by_key(f);
        vec.into_iter()
    }
}

impl<I: Iterator> ExtendedIter for I {}

pub trait ExtendedTup<I>: Sized {
    fn map<O, F: Fn(I) -> O>(self, f: F) -> (O, O);
    fn iter<'a>(&'a self) -> core::array::IntoIter<&'a I, 2>;
    fn into_iter(self) -> core::array::IntoIter<I, 2>;
}

impl<I> ExtendedTup<I> for (I, I) {
    fn map<O, F: Fn(I) -> O>(self, f: F) -> (O, O) {
        (f(self.0), f(self.1))
    }
    fn iter<'a>(&'a self) -> core::array::IntoIter<&'a I, 2> {
        [&self.0, &self.1].into_iter()
    }
    fn into_iter(self) -> core::array::IntoIter<I, 2> {
        [self.0, self.1].into_iter()
    }
}

pub fn get_input(day: usize) -> String {
    std::fs::read_to_string(format!("inputs/day{day}.in")).unwrap()
}

#[macro_export]
macro_rules! input {
    () => {
        std::fs::read_to_string(file!().replace("src/bin", "inputs").replace(".rs", ".in")).unwrap()
    };
}

#[cfg(test)]
mod tests {
    use super::*;
}
