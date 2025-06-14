pub mod bubble;
pub mod counting;
pub mod heap;
pub mod insertion;
pub mod merge;
pub mod quick;
pub mod selection;
pub mod shell;

pub trait Sort<T: Ord> {
    fn sort(&self, arr: Vec<T>) -> Vec<T>;
}

#[cfg(test)]
mod tests {
    use crate::sort::Sort;

    pub fn test_sort(sorter: impl Sort<i32>) {
        let arr = vec![5, 2, 9, 1, 5, 6];
        let sorted = sorter.sort(arr);
        assert_eq!(sorted, [1, 2, 5, 5, 6, 9]);
    }

    pub fn test_sort_empty(sorter: impl Sort<i32>) {
        let arr = Vec::new();
        let sorted = sorter.sort(arr);
        assert_eq!(sorted, Vec::<i32>::new());
    }

    pub fn test_sort_single_element(sorter: impl Sort<i32>) {
        let arr = vec![42];
        let sorted = sorter.sort(arr);
        assert_eq!(sorted, vec![42]);
    }
}
