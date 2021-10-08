use std::cmp::PartialOrd;

fn main() {
    let array = ["d", "gh", "jo", "ye", "nc", "ae"];
    println!("{:?}", array);
    let sorted = merge_sort(&array);
    println!("{:?}", sorted);
}

fn merge_sort<T: Clone + Copy + PartialOrd>(array: &[T]) -> Vec<T> {
    let len = array.len();
    if len == 1 {
        return Vec::from(array);
    }
    if len == 2 {
        let mut clone = Vec::from(array);
        if array[1] < array[0] {
            clone.swap(0, 1);
        }
        return clone;
    }
    let mid_index = len / 2;
    let left = merge_sort(&array[0..mid_index]);
    let right = merge_sort(&array[mid_index..]);
    let mut result = Vec::with_capacity(len);
    let mut left_index = 0;
    let mut right_index = 0;
    for _ in 0..len {
        if right_index >= right.len() || left_index < left.len() && left[left_index] <= right[right_index] {
            result.push(left[left_index]);
            left_index += 1;
        } else {
            result.push(right[right_index]);
            right_index += 1;
        }
    }
    return result;
}