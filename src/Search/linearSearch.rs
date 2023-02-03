
pub fn linear_search(numbers: Vec<i32>, target: i32)-> i32 {
    let mut i = 0;
    for num in numbers{
        if num == target {
            return i;
        }
        i += 1;
    }
    return -1;
}