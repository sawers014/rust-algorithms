/*

    Sort the first half, recursively. (1.1)
    Sort the second half, recursively. (1.2)
    Find the maximum of the whole array by comparing the results of 1.1 and 1.2, and place it at the end of the list. (1.3)
    Sort the entire list (except for the maximum now at the end), recursively. (2)

*/

use std::cmp::PartialOrd;

fn sort<T: PartialOrd>(items: &mut[T], start: usize, end: usize){
    if start >= end {
        return;
    }

    let middle = (start + end) / 2;

    sort(items, start, middle);
    sort(items, middle + 1, end);
    
    if items[middle] > items[end] {
        items.swap(end, middle);
    }
    
    sort(items, start, end - 1);
}

pub fn slow_sort<T: PartialOrd>(items: &mut[T]) {
    if items.is_empty() {
        return;
    }
    
    let len = items.len();
    
    sort(items, 0, len - 1);
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::is_sorted;

    #[test]
    fn sort(){
        let mut vec = vec![6, 1, 3, 2, 5, 4];
        slow_sort(&mut vec);
        
        assert!(is_sorted(&vec));
    }

    #[test]
    fn sorted(){
        let mut vec = vec![1, 2, 3, 4, 5, 6, 7];
        slow_sort(&mut vec);

        assert!(is_sorted(&vec));
    }
    
    #[test]
    fn empty(){
        let mut vec: Vec<i32> = vec![];
        slow_sort(&mut vec);

        assert!(is_sorted(&vec));
    }
}