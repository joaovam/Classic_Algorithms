pub fn merge_sort(mut numbers: &Vec<i32>) -> Vec<i32> {
    merge_sort_private(numbers)
}

fn merge_sort_private(numbers: &Vec<i32>) -> Vec<i32> {
    if numbers.len() < 2 {
        return numbers.to_vec();
    } else {
        let size = numbers.len() / 2;
        let left = merge_sort_private(&numbers[0..size].to_vec());

        let right = merge_sort_private(&numbers[size..].to_vec());

        //println!("P {}, Q {},R {}", p, q, r);
        let merged = merge(&left, &right);
        merged
    }
}

fn merge(left: &Vec<i32>, right: &Vec<i32>) -> Vec<i32> {
    let mut merged: Vec<i32> = Vec::new();

    let mut i = 0;
    let mut j = 0;

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            merged.push(left[i]);
            i += 1;
        } else {
            merged.push(right[j]);
            j += 1
        }
    }
    while i < left.len() {
        merged.push(left[i]);
        i += 1;
    }

    while j < right.len() {
        merged.push(right[j]);
        j += 1;
    }
    merged
}
