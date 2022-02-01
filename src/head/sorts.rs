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

pub mod mod_merge {
    use std::sync::mpsc;
    use std::thread;

    pub fn merge_sort(vec: &Vec<i32>) -> Vec<i32> {
        if vec.len() < 2 {
            vec.to_vec()
        } else {
            let size = vec.len() / 2; 
            let vec_copy: Vec<i32> = vec.to_vec(); // copy vec cause ownership
            let (tx, rx) = mpsc::channel(); // create channel

            let handle = thread::spawn(move || { // create new thread
                let left = merge_sort(&vec_copy[0..size].to_vec()); // recurse
                tx.send(left).unwrap(); // send new array
            });

            let left = rx.recv().unwrap(); // recieve new array

            let size = vec.len() / 2;
            let vec_copy: Vec<i32> = vec.to_vec(); // copy vec cause ownership
            let (tx, rx) = mpsc::channel(); // re-establish channel

            let handle2 = thread::spawn(move || { // spawn new thread
                let right = merge_sort(&vec_copy[size..].to_vec()); // recurse
                tx.send(right).unwrap(); // send new array
            });

            let right = rx.recv().unwrap();
            
            handle.join().unwrap();
            handle2.join().unwrap();

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


