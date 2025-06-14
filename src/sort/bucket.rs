use crate::sort::Sort;

#[derive(Debug, Clone, Copy)]
pub struct BucketSort;

pub trait BucketSortHelper: Ord {
    const MIN: Self;
    const MAX: Self;
    fn calc_len(max: Self, min: Self) -> usize;
}

impl BucketSortHelper for i32 {
    const MIN: Self = i32::MIN;
    const MAX: Self = i32::MAX;

    fn calc_len(max: Self, min: Self) -> usize {
        (max - min) as usize
    }
}

const BUCKET_SIZE: usize = 5;
impl<T: Ord + BucketSortHelper + Copy> Sort<T> for BucketSort {
    fn sort(&self, arr: Vec<T>) -> Vec<T> {
        if arr.is_empty() {
            return arr;
        }
        let (min, max) = arr
            .iter()
            .fold((T::MAX, T::MIN), |(min, max), x| (min.min(*x), max.max(*x)));
        let bucket_count = T::calc_len(max, min) / BUCKET_SIZE + 1;
        let mut buckets = vec![vec![]; bucket_count];
        for num in arr {
            buckets[T::calc_len(num, min) / BUCKET_SIZE].push(num);
        }
        let mut res = Vec::new();
        for mut bucket in buckets {
            bucket.sort(); // Using built-in sort for simplicity
            res.extend(bucket);
        }
        res
    }
}

#[cfg(test)]
mod test {
    use crate::sort::tests::{test_sort, test_sort_empty, test_sort_single_element};

    use super::*;

    #[test]
    fn test_bucket_sort() {
        let sorter = BucketSort;
        test_sort(sorter);
        test_sort_empty(sorter);
        test_sort_single_element(sorter);
    }
}
