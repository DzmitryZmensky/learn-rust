use std::thread::{self};
use num_cpus;

pub fn quick_sort_parallel<T: Send + PartialOrd + Copy>(slice: &mut [T]) {
    let cores = num_cpus::get();
    quick_sort_recursive(slice, cores, 1);
}

fn quick_sort_recursive<T: Send + PartialOrd + Copy>(slice: &mut [T], cores: usize, level: u32) {
    if slice.len() <= 1 {
        return;
    }

    let (left, right) = partition(slice);
    if level < 10 && 2_u32.pow(level) as usize <= cores {
        thread::scope(|scope| {
            let join_handle = scope.spawn(||{
                quick_sort_recursive(right, cores, level + 1);
            });

            quick_sort_recursive(left, cores, level + 1);
            join_handle.join().unwrap();
        });
    } else {
        quick_sort_recursive(left, cores, level + 1);
        quick_sort_recursive(right, cores, level + 1);
    }
}

fn partition<T: PartialOrd + Copy>(slice: &mut [T]) -> (&mut [T], &mut [T]) {
    if slice.len() <=1 {
        panic!()
    }

    let partition_val_index = slice.len() / 2;
    let partition_val = slice[partition_val_index];
    
    let mut write_index = 0;
    slice.swap(partition_val_index, slice.len() - 1);
    for read_index in 0..slice.len() - 1 {
        if slice[read_index] <= partition_val {
            if write_index != read_index {
                slice.swap(read_index, write_index);
            }
            write_index += 1;
        }
    }

    slice.swap(write_index, slice.len() - 1);

    let (l, r) = slice.split_at_mut(write_index);
    return (l, &mut r[1..]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arbitrary_data() {
        let mut v = [5,3,6,4,2,1,7];
        quick_sort_parallel(&mut v);
        let expected = [1,2,3,4,5,6,7];
        assert_eq!(expected, v);
    }

    #[test]
    fn already_sorted_data() {
        let mut v = [1,2,3,4,5,6,7];
        quick_sort_parallel(&mut v);
        let expected = [1,2,3,4,5,6,7];
        assert_eq!(expected, v);
    }

    #[test]
    fn sorted_in_reverse_data() {
        let mut v = [7,6,5,4,3,2,1];
        quick_sort_parallel(&mut v);
        let expected = [1,2,3,4,5,6,7];
        assert_eq!(expected, v);
    }

    #[test]
    fn all_duplicates() {
        let mut v = [1,1,1,1,1];
        quick_sort_parallel(&mut v);
        let expected = [1,1,1,1,1];
        assert_eq!(expected, v);
    }

    #[test]
    fn some_duplicates() {
        let mut v = [3,1,2,3,2];
        quick_sort_parallel(&mut v);
        let expected = [1,2,2,3,3];
        assert_eq!(expected, v);
    }

    #[test]
    fn perf_on_large_dataset() {
        let mut v = vec![0; 20 * 1000 * 1000];
        let min_value = i32::MIN;
        let max_value = i32::MAX;
        random_number::random_fill!(v, min_value..max_value);
         
        quick_sort_parallel(&mut v); // it takes approximately 20 seconds on 4 core machine
        assert!(is_sorted(&v));
    }

    fn is_sorted<T: PartialOrd>(v:&Vec<T>) -> bool {
        for index in 1..v.len() {
            if v[index-1] > v[index] {
                return false;
            }
        };

        true
    }
}