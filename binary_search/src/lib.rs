pub fn bsearch<T: PartialOrd>(v: &[T], value:T) -> Option<usize> {
    if v.len() == 0 { return None }
    let mut l: usize = 0;
    let mut r = v.len() - 1;
    while l < r {
        let m = l + (r - l) / 2;
        if v[m] >= value {
            r = m;
        }
        else {
            l = m + 1;
        }
    }
    if v[l] == value {
        Some(l)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::bsearch;

    #[test]
    fn empty_dataset() {
        let v = [];
        let i = 2;
        let result = bsearch(&v, i);
        assert_eq!(None, result);
    }

    #[test]
    fn middle_location() {
        let v = [1,2,3];
        let i = 2;
        let result = bsearch(&v, i);
        assert_eq!(Some(1), result);
    }

    #[test]
    fn middle_location_duplicates() {
        let v = [1,2,2,3];
        let i = 2;
        let result = bsearch(&v, i);
        assert_eq!(Some(1), result);
    }

    #[test]
    fn leftmost_location() {
        let v = [1,2,3];
        let i = 1;
        let result = bsearch(&v, i);
        assert_eq!(Some(0), result);
    }

    #[test]
    fn leftmost_location_duplicates() {
        let v = [1,1,2,3];
        let i = 1;
        let result = bsearch(&v, i);
        assert_eq!(Some(0), result);
    }

    #[test]
    fn rightmost_location() {
        let v = [1,2,3];
        let i = 3;
        let result = bsearch(&v, i);
        assert_eq!(Some(2), result);
    }

    #[test]
    fn rightmost_location_duplicates() {
        let v = [1,2,3,3];
        let i = 3;
        let result = bsearch(&v, i);
        assert_eq!(Some(2), result);
    }

    #[test]
    fn missing_in_the_middle() {
        let v = [1,3];
        let i = 2;
        let result = bsearch(&v, i);
        assert_eq!(None, result);
    }

    #[test]
    fn missing_greater_value() {
        let v = [1,3];
        let i = 4;
        let result = bsearch(&v, i);
        assert_eq!(None, result);
    }

    #[test]
    fn missing_smaller_value() {
        let v = [1,3];
        let i = 0;
        let result = bsearch(&v, i);
        assert_eq!(None, result);
    }
}
