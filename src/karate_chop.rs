// http://codekata.com/kata/kata02-karate-chop/
pub fn chop(value: i32, numbers: Vec<i32>) -> i32 {
    if numbers.is_empty() {
        return -1;
    }

    let length = numbers.len();
    let mut index = 0;

    loop {
        index = if value < numbers[index] {
            index / 2
        } else {
            index + (length - index) / 2
        };

        if value == numbers[index] {
            return index as i32;
        }

        if index == 0 || index == length - 1 {
            return -1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chop() {
        assert_eq!(-1, chop(3, vec![]));
        assert_eq!(-1, chop(3, vec![1]));
        assert_eq!(0, chop(1, vec![1]));
        assert_eq!(0, chop(1, vec![1, 3, 5]));
        assert_eq!(1, chop(3, vec![1, 3, 5]));
        assert_eq!(2, chop(5, vec![1, 3, 5]));
        assert_eq!(-1, chop(0, vec![1, 3, 5]));
        assert_eq!(-1, chop(2, vec![1, 3, 5]));
        assert_eq!(-1, chop(4, vec![1, 3, 5]));
        assert_eq!(-1, chop(6, vec![1, 3, 5]));
        assert_eq!(0, chop(1, vec![1, 3, 5, 7]));
        assert_eq!(1, chop(3, vec![1, 3, 5, 7]));
        assert_eq!(2, chop(5, vec![1, 3, 5, 7]));
        assert_eq!(3, chop(7, vec![1, 3, 5, 7]));
        assert_eq!(-1, chop(0, vec![1, 3, 5, 7]));
        assert_eq!(-1, chop(2, vec![1, 3, 5, 7]));
        assert_eq!(-1, chop(6, vec![1, 3, 5, 7]));
        assert_eq!(-1, chop(8, vec![1, 3, 5, 7]));
    }
}
