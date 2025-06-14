use crate::sort::Sort;

pub struct CountingSort;

impl<T> Sort<T> for CountingSort
where
    T: Ord + Copy + Into<usize> + From<usize>,
{
    fn sort(&self, mut _arr: Vec<T>) -> Vec<T> {
        todo!()
    }
}
