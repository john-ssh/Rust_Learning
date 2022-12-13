// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

use std::fmt::Debug;
// O(n^2)
pub fn bubble_sort<T: PartialOrd + Debug> (v:&mut [T]) {
    for p in 0 ..v.len() {
        let mut sorted = true;
        for i in 0..(v.len() -1) - p {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
                sorted = false;
            }
        }
        println!("{:?}", v);
        if sorted {
            return;
        }   
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bubble_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        bubble_sort(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13]);
    }

}




    //     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
