
pub fn selection_sort(mut numbers: Vec<i32>) -> Vec<i32>{


    for i in 0..numbers.len() - 1{//do not iterate over the last element
        let mut smaller = i;
        for j in i..numbers.len()  {
            if numbers[smaller] > numbers[j] {
                smaller = j;
            }

        }
        (numbers[i], numbers[smaller]) = (numbers[smaller], numbers[i]);
    }
    numbers
}