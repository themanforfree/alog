use crate::sort::Sort;

#[derive(Debug, Clone, Copy)]
pub struct SelectionSort;

impl Sort for SelectionSort {
    fn sort<T: Ord + Copy>(&self, mut arr: Vec<T>) -> Vec<T> {
        let len = arr.len();
        for i in 0..len {
            let mut min_index = i;
            for j in i + 1..len {
                if arr[j] < arr[min_index] {
                    min_index = j;
                }
            }
            arr.swap(i, min_index);
        }
        arr
    }
}

#[cfg(test)]
mod test {
    use crate::sort::tests::{test_sort, test_sort_empty, test_sort_single_element};

    use super::*;
    #[test]
    fn test_selection_sort() {
        let sorter = SelectionSort;
        test_sort(sorter);
        test_sort_empty(sorter);
        test_sort_single_element(sorter);
    }
}
