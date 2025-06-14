pub mod bubble;
pub mod insertion;
pub mod selection;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    fn get_all_sort_fns() -> HashMap<&'static str, fn(&mut [i32])> {
        let mut sort_fns: HashMap<&'static str, fn(&mut [i32])> = HashMap::new();
        sort_fns.insert("insertion_sort", super::insertion::insertion_sort);
        sort_fns.insert("bubble_sort", super::bubble::bubble_sort);
        sort_fns.insert("selection_sort", super::selection::selection_sort);
        sort_fns
    }

    #[test]
    fn test_sort() {
        for (name, sort_fn) in get_all_sort_fns() {
            let mut arr = [5, 2, 9, 1, 5, 6];
            sort_fn(&mut arr);
            assert_eq!(arr, [1, 2, 5, 5, 6, 9], "Failed on {}", name);
        }
    }

    #[test]
    fn test_sort_empty() {
        for (name, sort_fn) in get_all_sort_fns() {
            let mut arr: [i32; 0] = [];
            sort_fn(&mut arr);
            assert_eq!(arr, [], "Failed on {}", name);
        }
    }

    #[test]
    fn test_sort_single_element() {
        for (name, sort_fn) in get_all_sort_fns() {
            let mut arr = [42];
            sort_fn(&mut arr);
            assert_eq!(arr, [42], "Failed on {}", name);
        }
    }
}
