use crate::sort::Sort;

#[derive(Debug, Clone, Copy)]
pub struct ShellSort;

impl Sort for ShellSort {
    fn sort<T: Ord + Copy>(&self, mut arr: Vec<T>) -> Vec<T> {
        let len = arr.len();
        let mut gap = len / 2;

        while gap > 0 {
            for i in gap..len {
                let temp = arr[i];
                let mut j = i;

                while j >= gap && arr[j - gap] > temp {
                    arr[j] = arr[j - gap];
                    j -= gap;
                }
                arr[j] = temp;
            }
            gap /= 2;
        }
        arr
    }
}

#[cfg(test)]
mod test {
    use crate::sort::tests::{test_sort, test_sort_empty, test_sort_single_element};

    use super::*;
    #[test]
    fn test_shell_sort() {
        let sorter = ShellSort;
        test_sort(sorter);
        test_sort_empty(sorter);
        test_sort_single_element(sorter);
    }
}
