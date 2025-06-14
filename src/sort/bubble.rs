/// Time Complexity: O(n^2)
///
/// Space Complexity: O(1)
pub fn bubble_sort<T: Ord + Copy>(arr: &mut [T]) {
    for i in 0..arr.len() {
        let mut swapped = false;
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break; // No swaps means the array is sorted
        }
    }
}
