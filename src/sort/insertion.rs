/// Time Complexity: O(n^2)
///
/// Space Complexity: O(1)
pub fn insertion_sort<T: Ord + Copy>(arr: &mut [T]) {
    let len = arr.len();
    for i in 1..len {
        let key = arr[i];
        let mut j = i;

        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }

        arr[j] = key;
    }
}
