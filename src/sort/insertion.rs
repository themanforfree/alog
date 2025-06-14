use crate::sort::Sort;

#[derive(Debug, Clone, Copy)]
pub struct InsertionSort;

impl Sort for InsertionSort {
    fn sort<T: Ord + Copy>(&self, mut arr: Vec<T>) -> Vec<T> {
        let len = arr.len();
        for i in 1..len {
            let key = arr[i];
            let mut j = i;

            while j > 0 && arr[j - 1] > key {
                arr[j] = arr[j - 1];
                j -= 1;
            }

            arr[j] = key;
        }
        arr
    }
}

#[cfg(test)]
mod test {
    use crate::sort::tests::{test_sort, test_sort_empty, test_sort_single_element};

    use super::*;
    #[test]
    fn test_insertion_sort() {
        let sorter = InsertionSort;
        test_sort(sorter);
        test_sort_empty(sorter);
        test_sort_single_element(sorter);
    }
}
