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

pub mod ModMerge {
    pub fn merge_sort(vec: &Vec<i32>) -> Vec<i32> {
        if vec.len() < 2 {
            vec.to_vec()
        } else {
            let size = vec.len() / 2;
            let left = merge_sort(&vec[0..size].to_vec());
            let right = merge_sort(&vec[size..].to_vec());
            let merged = merge(&left, &right);

            merged
        }
    }

    fn merge(left: &Vec<i32>, right: &Vec<i32>) -> Vec<i32> {
        let mut l = 0;
        let mut r = 0;
        let mut merged: Vec<i32> = Vec::new();

        while l < left.len() && r < right.len() {
            if left[l] < right[r] {
                merged.push(left[l]);
                l = l + 1;
            } else {
                merged.push(right[r]);
                r = r + 1;
            }
        }
        
        if l < left.len() {
            while l < left.len() {
                merged.push(left[l]);
                l = l +1;
            }
        }

        if r < right.len() {
            while r < right.len() {
                merged.push(right[r]);
                r = r + 1;
            }
        }
        
        merged
    }
}


