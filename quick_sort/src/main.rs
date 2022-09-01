fn main() {
    let mut v = [5,3,6,4,2,1,7];
    println!("{:?}", &v);
    quick_sort(&mut v);
    println!("{:?}", &v);
}

fn quick_sort(slice: &mut [i32]) {
    if slice.len() <= 1 {
        return;
    }

    let (left, right) = partition(slice);
    quick_sort(left);
    quick_sort(right);
}

fn partition(slice: &mut [i32]) -> (&mut [i32], &mut [i32]) {
    println!("partitioning: {:?}", &slice);
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
    //(&mut slice[..write_index], &mut slice[write_index + 1 ..]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        let mut v = [5,3,6,4,2,1,7];
        quick_sort(&mut v);
        let expected = [1,2,3,4,5,6,7];
        assert_eq!(expected, v);
    }
}