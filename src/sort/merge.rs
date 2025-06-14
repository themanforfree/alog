use crate::sort::Sort;

#[derive(Debug, Clone, Copy)]
pub struct MergeSort;

impl Sort for MergeSort {
    fn sort<T: Ord + Copy>(&self, arr: Vec<T>) -> Vec<T> {
        if arr.len() <= 1 {
            return arr;
        }

        let mid = arr.len() / 2;
        let left = self.sort(arr[..mid].to_vec());
        let right = self.sort(arr[mid..].to_vec());

        Self::merge(left, right)
    }
}

impl MergeSort {
    fn merge<T: Ord + Copy>(left: Vec<T>, right: Vec<T>) -> Vec<T> {
        let mut result = Vec::with_capacity(left.len() + right.len());
        let mut i = 0;
        let mut j = 0;

        while i < left.len() && j < right.len() {
            if left[i] <= right[j] {
                result.push(left[i]);
                i += 1;
            } else {
                result.push(right[j]);
                j += 1;
            }
        }

        // Append remaining elements
        result.extend_from_slice(&left[i..]);
        result.extend_from_slice(&right[j..]);

        result
    }
}

#[cfg(test)]
mod test {
    use crate::sort::tests::{test_sort, test_sort_empty, test_sort_single_element};

    use super::*;

    #[test]
    fn test_merge_sort() {
        let sorter = MergeSort;
        test_sort(sorter);
        test_sort_empty(sorter);
        test_sort_single_element(sorter);
    }
}
