/*

The algorithm is defined as follows:
    If the value at the start is larger than the value at the end, swap them.
    If there are 3 or more elements in the list, then:
        Stooge sort the initial 2/3 of the list
        Stooge sort the final 2/3 of the list
        Stooge sort the initial 2/3 of the list again

*/

use std::cmp::PartialOrd;

fn sort<T: PartialOrd>(items: &mut[T], start: usize, end: usize){
    if items[start] > items[end] {
        items.swap(start, end)
    }

    if start + 1 >= end {
        return;
    }

    let x = (end - start + 1) / 3;

    sort(items, start, end - x);
    
    sort(items, start + x, end);
    
    sort(items, start, end - x);
}

pub fn stooge_sort<T: PartialOrd>(items: &mut[T]) {
    if items.is_empty() {
        return;
    }

    let len = items.len();

    sort(items, 0, len - 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::is_sorted;

    #[test]
    fn sort(){
        let mut vec = vec![6, 1, 3, 2, 5, 4];
        stooge_sort(&mut vec);

        assert!(is_sorted(&vec));
    }

    #[test]
    fn sorted(){
        let mut vec = vec![1, 2, 3, 4, 5, 6, 7];
        stooge_sort(&mut vec);

        assert!(is_sorted(&vec));
    }
    
    #[test]
    fn empty(){
        let mut vec: Vec<i32> = vec![];
        stooge_sort(&mut vec);

        assert!(is_sorted(&vec));
    }
}