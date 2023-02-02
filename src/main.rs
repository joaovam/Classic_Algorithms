#[path = "Search/insertionSort.rs"] mod insertion_sort;

fn main() {

    let numbers = vec![4, 1, 2, 3, 2, 1];

    println!("new Vector {:?}", insertion_sort::insertion_sort(numbers));

    
}
