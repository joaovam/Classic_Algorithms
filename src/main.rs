#[path = "Sort/insertionSort.rs"] mod insertion_sort;
#[path = "Sort/selectionSort.rs"] mod selection_sort;
#[path = "Search/linearSearch.rs"] mod linear_search;



use crate::insertion_sort::insertion_sort;
use crate::linear_search::linear_search;
use crate::selection_sort::selection_sort;

fn main() {

    let mut numbers = vec![1,1,1,1,1,5,6,7,1];

    numbers = selection_sort(numbers);

    println!("Result {:?}", numbers);

    
}
