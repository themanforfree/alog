pub fn shell_sort<T: Ord + Copy>(arr: &mut [T]) {
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
}
