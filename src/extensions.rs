use crate::point::P;

pub fn num_len(num: u64) -> u32 {
    if num == 0 {
        return 1;
    }

    num.ilog10() + 1
}

pub fn split_num_in_two(num: u64) -> (u64, u64) {
    let split = 10_u64.pow(num_len(num) / 2);
    (num / split, num % split)
}

pub trait CollectionExtensions<T> {
    fn get_by(&self, index: i32) -> Option<&T>;
    fn get_mut_by(&mut self, index: i32) -> Option<&mut T>;
}

impl<T> CollectionExtensions<T> for [T] {
    fn get_by(&self, index: i32) -> Option<&T> {
        if index < 0 {
            return None;
        }

        let index = index as usize;
        self.get(index)
    }

    fn get_mut_by(&mut self, index: i32) -> Option<&mut T> {
        if index < 0 {
            return None;
        }

        let index = index as usize;
        self.get_mut(index)
    }
}

pub trait TableExtensions<T> {
    fn get_by_p(&self, index: P) -> Option<&T>;
    fn is_at_p(&self, index: P, required: &T) -> bool;
    fn get_mut_by_p(&mut self, index: P) -> Option<&mut T>;
    fn height(&self) -> usize;
    fn width(&self) -> usize;
}

impl<T: std::cmp::PartialEq> TableExtensions<T> for [Vec<T>] {
    fn get_by_p(&self, index: P) -> Option<&T> {
        self.get_by(index.y).and_then(|line| line.get_by(index.x))
    }

    fn is_at_p(&self, index: P, required: &T) -> bool {
        self.get_by_p(index).is_some_and(|value| value == required)
    }

    fn get_mut_by_p(&mut self, index: P) -> Option<&mut T> {
        self.get_mut_by(index.y)
            .and_then(|line| line.get_mut_by(index.x))
    }

    fn height(&self) -> usize {
        self.len()
    }

    fn width(&self) -> usize {
        self[0].len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_len() {
        assert_eq!(num_len(0), 1);
        assert_eq!(num_len(1), 1);

        assert_eq!(num_len(10), 2);
        assert_eq!(num_len(11), 2);
        assert_eq!(num_len(99), 2);

        assert_eq!(num_len(100), 3);
        assert_eq!(num_len(101), 3);
        assert_eq!(num_len(999), 3);
    }

    #[test]
    fn test_num_split() {
        assert_eq!(split_num_in_two(11), (1, 1));
        assert_eq!(split_num_in_two(1234), (12, 34));
    }
}
