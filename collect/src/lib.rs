pub fn bubble_sort(arr: &mut [i32]) {
    for _i in 0..arr.len() {
        for j in 0..arr.len()-1 {
            if arr[j] > arr[j+1] {
                let sub = arr[j];
                arr[j] = arr[j+1];
                arr[j+1] = sub;
            } 
        }
    }  
}