// Implements binary search in a slice of sorted elements, returns the found index if any.
pub fn bsearch<T: PartialOrd>(input: &[T], value:T) -> Option<usize> {
    if input.len() == 0 { return None }
    let mut left_index: usize = 0;
    let mut right_index = input.len() - 1;
    while left_index < right_index {
        let m = left_index + (right_index - left_index) / 2;
        if input[m] >= value {
            right_index = m;
        }
        else {
            left_index = m + 1;
        }
    }
    if input[left_index] == value {
        Some(left_index)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::bsearch;

    #[test]
    fn empty_dataset() {
        let input = [];
        let value = 2;
        let result = bsearch(&input, value);
        assert_eq!(None, result);
    }

    #[test]
    fn middle_location() {
        let input = [1,2,3];
        let value = 2;
        let result = bsearch(&input, value);
        assert_eq!(Some(1), result);
    }

    #[test]
    fn middle_location_duplicates() {
        let input = [1,2,2,3];
        let value = 2;
        let result = bsearch(&input, value);
        assert_eq!(Some(1), result);
    }

    #[test]
    fn leftmost_location() {
        let input = [1,2,3];
        let value = 1;
        let result = bsearch(&input, value);
        assert_eq!(Some(0), result);
    }

    #[test]
    fn leftmost_location_duplicates() {
        let input = [1,1,2,3];
        let value = 1;
        let result = bsearch(&input, value);
        assert_eq!(Some(0), result);
    }

    #[test]
    fn rightmost_location() {
        let input = [1,2,3];
        let value = 3;
        let result = bsearch(&input, value);
        assert_eq!(Some(2), result);
    }

    #[test]
    fn rightmost_location_duplicates() {
        let input = [1,2,3,3];
        let value = 3;
        let result = bsearch(&input, value);
        assert_eq!(Some(2), result);
    }

    #[test]
    fn missing_in_the_middle() {
        let input = [1,3];
        let value = 2;
        let result = bsearch(&input, value);
        assert_eq!(None, result);
    }

    #[test]
    fn missing_greater_value() {
        let input = [1,3];
        let value = 4;
        let result = bsearch(&input, value);
        assert_eq!(None, result);
    }

    #[test]
    fn missing_smaller_value() {
        let input = [1,3];
        let value = 0;
        let result = bsearch(&input, value);
        assert_eq!(None, result);
    }
}
