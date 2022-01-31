pub fn bubble_sort(array: &mut Vec<i32>) {
    let mut swapped: bool = true;
    while swapped {
        swapped = false;
        for i in 1..array.len() {
            if array[i-1] > array[i] {
                array.swap(i-1, i);
                swapped = true;
            }
        }
    }
}
