use std::{cmp::Reverse, collections::BinaryHeap};

use crate::sort::Sort;

#[derive(Debug, Clone, Copy)]
pub struct HeapSort;

impl Sort for HeapSort {
    fn sort<T: Ord + Copy>(&self, arr: Vec<T>) -> Vec<T> {
        let mut heap = BinaryHeap::new(); // TODO: implement a custom heap
        for item in arr {
            heap.push(Reverse(item));
        }
        let mut sorted = Vec::with_capacity(heap.len());
        while let Some(Reverse(item)) = heap.pop() {
            sorted.push(item);
        }
        sorted
    }
}

#[cfg(test)]
mod test {
    use crate::sort::tests::{test_sort, test_sort_empty, test_sort_single_element};

    use super::*;
    #[test]
    fn test_heap_sort() {
        let sorter = HeapSort;
        test_sort(sorter);
        test_sort_empty(sorter);
        test_sort_single_element(sorter);
    }
}
