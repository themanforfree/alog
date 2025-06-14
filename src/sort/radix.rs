use crate::sort::Sort;

#[derive(Debug, Clone, Copy)]
pub struct RadixSort;

impl Sort<i32> for RadixSort {
    fn sort(&self, mut arr: Vec<i32>) -> Vec<i32> {
        if arr.is_empty() {
            return arr;
        }
        let size = arr.iter().max().copied().unwrap().to_string().len();
        for i in 0..size {
            let mut buckets = vec![Vec::new(); 10];
            for num in &arr {
                let digit = (num / 10_i32.pow(i as u32)) % 10;
                buckets[digit as usize].push(*num);
            }
            arr.clear();
            for bucket in buckets {
                arr.extend(bucket);
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
    fn test_radix_sort() {
        let sorter = RadixSort;
        test_sort(sorter);
        test_sort_empty(sorter);
        test_sort_single_element(sorter);
    }
}
