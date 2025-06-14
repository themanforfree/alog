use crate::sort::Sort;

#[derive(Debug, Clone, Copy)]
pub struct CountingSort;

pub trait CountingSortHelper: Ord {
    fn min_value() -> Self;
    fn max_value() -> Self;
    fn calc_len(max: Self, min: Self) -> usize;
    fn calc_index(value: Self, min: Self) -> usize;
    fn calc_value(index: usize, min: Self) -> Self;
}

impl CountingSortHelper for i32 {
    fn min_value() -> Self {
        i32::MIN
    }

    fn max_value() -> Self {
        i32::MAX
    }

    fn calc_len(max: Self, min: Self) -> usize {
        (max - min + 1) as usize
    }

    fn calc_index(value: Self, min: Self) -> usize {
        (value - min) as usize
    }

    fn calc_value(index: usize, min: Self) -> Self {
        index as i32 + min
    }
}

impl<T: Ord + CountingSortHelper + Copy> Sort<T> for CountingSort {
    fn sort(&self, arr: Vec<T>) -> Vec<T> {
        if arr.is_empty() {
            return arr;
        }
        let (min, max) = arr
            .iter()
            .fold((T::max_value(), T::min_value()), |(min, max), x| {
                (min.min(*x), max.max(*x))
            });
        let len = T::calc_len(max, min);
        let mut count = vec![0; len];
        for n in &arr {
            count[T::calc_index(*n, min)] += 1;
        }
        let mut res = Vec::with_capacity(arr.len());
        for (i, &c) in count.iter().enumerate() {
            res.extend(std::iter::repeat_n(T::calc_value(i, min), c));
        }
        res
    }
}

#[cfg(test)]
mod test {
    use crate::sort::tests::{test_sort, test_sort_empty, test_sort_single_element};

    use super::*;

    #[test]
    fn test_counting_sort() {
        let sorter = CountingSort;
        test_sort(sorter);
        test_sort_empty(sorter);
        test_sort_single_element(sorter);
    }
}
