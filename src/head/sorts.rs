pub fn bubble_sort(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec_new = vec.to_vec();
    let mut swapped: bool = true;
    while swapped {
        swapped = false;
        for i in 1..vec_new.len() {
            if vec_new[i-1] > vec_new[i] {
                vec_new.swap(i-1, i);
                swapped = true;
            }
        }
    }
    vec_new
}
