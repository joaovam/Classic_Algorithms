#[path = "Sort/insertionSort.rs"]
mod insertion_sort;
#[path = "Sort/mergeSort.rs"]
mod merge_sort;
#[path = "Sort/selectionSort.rs"]
mod selection_sort;

#[path = "Search/linearSearch.rs"]
mod linear_search;

use crate::insertion_sort::insertion_sort;
use crate::linear_search::linear_search;
use crate::merge_sort::merge_sort;
use crate::selection_sort::selection_sort;

fn main() {
    let mut numbers = &vec![5, 4, 3, 2, 1];

    //selection_sort(numbers);

    println!("Result {:?}", merge_sort(numbers));
}
