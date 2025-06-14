use crate::sort::Sort;

#[derive(Debug, Clone, Copy)]
pub struct QuickSort;

impl<T: Ord + Copy> Sort<T> for QuickSort {
    fn sort(&self, mut arr: Vec<T>) -> Vec<T> {
        if arr.is_empty() {
            return arr;
        }
        let len = arr.len();
        self.quick_sort_inner(&mut arr, 0, len - 1);
        arr
    }
}

impl QuickSort {
    fn partition<T: Ord + Copy>(&self, arr: &mut [T], low: usize, high: usize) -> usize {
        let pivot = arr[low];
        let (mut i, mut j) = (low, high);
        while i < j {
            while i < j && arr[j] >= pivot {
                j -= 1;
            }
            while i < j && arr[i] <= pivot {
                i += 1;
            }
            if i < j {
                arr.swap(i, j);
            }
        }
        arr.swap(low, i);
        i
    }

    fn quick_sort_inner<T: Ord + Copy>(&self, arr: &mut [T], low: usize, high: usize) {
        if low < high {
            let pivot_index = self.partition(arr, low, high);
            self.quick_sort_inner(arr, low, pivot_index);
            self.quick_sort_inner(arr, pivot_index + 1, high);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::sort::tests::{test_sort, test_sort_empty, test_sort_single_element};

    use super::*;

    #[test]
    fn test_quick_sort() {
        let sorter = QuickSort;
        test_sort(sorter);
        test_sort_empty(sorter);
        test_sort_single_element(sorter);
    }
}
