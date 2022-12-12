
pub trait ExtendedIter: Iterator + Sized {
    fn pop(&mut self) -> Self::Item {
        self.next().unwrap()
    }

    fn pop_back(&mut self) -> Self::Item 
        where Self: DoubleEndedIterator
    {
        self.next_back().unwrap()
    }

    fn sort(self) -> std::vec::IntoIter<<Self as Iterator>::Item>
        where Self::Item: Ord
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


pub trait TupleMap<I>: Sized {
    fn map<O, F: Fn(I) -> O>(self, f: F) -> (O, O);
}

impl<I> TupleMap<I> for (I, I) {
    fn map<O, F: Fn(I) -> O>(self, f: F) -> (O, O) {
        (f(self.0), f(self.1))
    }
} 

pub fn get_input(day: usize) -> String {
    std::fs::read_to_string(format!("inputs/day{day}.in")).unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;
}
