use std::cmp::PartialOrd;

pub fn is_sorted<T: PartialOrd>(items: &[T]) -> bool {
    if items.is_empty() {
        return true;
    }

    let mut previous = &items[0];

    for item in items.iter().skip(1) {
        if previous > item {
            return false;
        }

        previous = item;
    }

    return true;
}

#[cfg(test)]
mod tests{
    use super::is_sorted;

    #[test]
    fn sorted(){
        assert_eq!(is_sorted(&[1, 2, 3, 19, 4]), false);
        assert_eq!(is_sorted(&[1, 2, 3, 4]), true);

        assert_eq!(is_sorted(&[] as &[i32]), true);
    }
}