
pub fn insertion_sort( mut numbers: Vec<i32>) -> Vec<i32>{
    

    for i in 1..numbers.len(){

        let key = numbers[i];

        let mut index = i;

        while  index > 0 && key < numbers[index -1]{
            numbers[index] = numbers[index -1];
            index -= 1;
        }
        numbers[index] = key;
        
    }

    numbers
}