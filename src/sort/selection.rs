/// Time Complexity: O(n^2)
///
/// Space Complexity: O(1)
pub fn selection_sort<T: Ord + Copy>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        let mut min_index = i;
        for j in i + 1..len {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        arr.swap(i, min_index);
    }
}
