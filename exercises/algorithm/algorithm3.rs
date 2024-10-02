/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/


fn sort<T: std::cmp::PartialOrd>(array: &mut [T]){
	//TODO
    fn q_sort<T: std::cmp::PartialOrd>(array: &mut [T], left: usize, right: usize) -> usize {
        if  left < right {
            //let pivot = &array[left];
            let mut i = left;
            let mut j = right;
            while i < j {
                while i < j && array[j] >= array[i] {
                    j -= 1;
                }
                if i < j{
                    array.swap(i, j);   
                }
                while i < j && array[i] < array[j] {
                    i += 1;
                }
                if i < j{
                    array.swap(i, j);
                }
            }
            return i;
        }
        return left;
    }
    fn q_sort_recursive<T: std::cmp::PartialOrd>(array: &mut [T], left: usize, right: usize) {
        if left < right {
            let mid = q_sort(array, left, right);
            // 这里一定要比较一下 否则mid:usize可能溢出
            if left < mid {
                q_sort_recursive(array, left, mid - 1);
            }
            if mid + 1 < right {
                q_sort_recursive(array, mid + 1, right);
            }
        }
    }
    q_sort_recursive(array, 0, array.len() - 1);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}