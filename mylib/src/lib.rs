// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

//! 这是包说明
//! 
//! 我的mylib
/// 给一个数加1
/// 
/// #Example
/// 
/// ```
/// let five = 5;
/// assert_eq!(6, mylib::add_one(5));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
