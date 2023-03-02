use std::cmp::PartialOrd;

pub fn bubble_sort<T: PartialOrd>(items: &mut[T]) {
    if items.is_empty() {
        return;
    }

    let len = items.len();

    for i in 0..(len - 1) {
        for j in 0..(len - i) {
            if j == len - 1 {
                continue;
            }
            if items[j] > items[j + 1] {
                items.swap(j, j + 1)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::is_sorted;

    #[test]
    fn sort(){
        let mut vec = vec![6, 1];
        bubble_sort(&mut vec);

        assert!(is_sorted(&vec));
    }

    #[test]
    fn sorted(){
        let mut vec = vec![1, 2];
        bubble_sort(&mut vec);

        assert!(is_sorted(&vec));
    }
    
    #[test]
    fn empty(){
        let mut vec: Vec<i32> = vec![];
        bubble_sort(&mut vec);

        assert!(is_sorted(&vec));
    }
}