use crate::sort::Sort;

#[derive(Debug, Clone, Copy)]
pub struct BubbleSort;

impl<T: Ord> Sort<T> for BubbleSort {
    fn sort(&self, mut arr: Vec<T>) -> Vec<T> {
        for i in 0..arr.len() {
            let mut swapped = false;
            for j in 0..arr.len() - 1 - i {
                if arr[j] > arr[j + 1] {
                    arr.swap(j, j + 1);
                    swapped = true;
                }
            }
            if !swapped {
                break; // No swaps means the array is sorted
            }
        }
        arr
    }
}

#[cfg(test)]
mod test {
    use crate::sort::tests::{test_sort, test_sort_empty, test_sort_single_element};

    use super::*;
    #[test]
    fn test_bubble_sort() {
        let sorter = BubbleSort;
        test_sort(sorter);
        test_sort_empty(sorter);
        test_sort_single_element(sorter);
    }
}
