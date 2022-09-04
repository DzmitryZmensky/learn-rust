pub fn bsearch(v: &[i32], i:i32) -> Option<usize> {
    let mut l:usize = 0;
    let mut r:usize = v.len() - 1;
    while l < r {
        let m = l + (r - l) / 2;
        if v[m] >= i {
            r = m;
        }
        else {
            l = m + 1;
        }
    }
    if v[l] == i {
        Some(l)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::bsearch;

    #[test]
    fn test1() {
        let v = [1,2,3];
        let i = 2;
        let result = bsearch(&v, i).unwrap();
        assert_eq!(1, result);
    }

    #[test]
    fn test2() {
        let v = [1,2,2,3];
        let i = 2;
        let result = bsearch(&v, i).unwrap();
        assert_eq!(1, result);
    }

    #[test]
    fn test3() {
        let v = [1,2,3];
        let i = 1;
        let result = bsearch(&v, i).unwrap();
        assert_eq!(0, result);
    }

    #[test]
    fn test4() {
        let v = [1,1,2,3];
        let i = 1;
        let result = bsearch(&v, i).unwrap();
        assert_eq!(0, result);
    }

    #[test]
    fn test5() {
        let v = [1,2,3];
        let i = 3;
        let result = bsearch(&v, i).unwrap();
        assert_eq!(2, result);
    }

    #[test]
    fn test6() {
        let v = [1,2,3,3];
        let i = 3;
        let result = bsearch(&v, i).unwrap();
        assert_eq!(2, result);
    }

    #[test]
    fn test7() {
        let v = [1,3];
        let i = 2;
        let result = bsearch(&v, i);
        assert_eq!(None, result);
    }

    #[test]
    fn test8() {
        let v = [1,3];
        let i = 4;
        let result = bsearch(&v, i);
        assert_eq!(None, result);
    }

    #[test]
    fn test9() {
        let v = [1,3];
        let i = 0;
        let result = bsearch(&v, i);
        assert_eq!(None, result);
    }
}
