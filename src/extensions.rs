use crate::point::P;

pub trait CollectionExtensions<T> {
    fn get_by(&self, index: i32) -> Option<&T>;
}

impl<T> CollectionExtensions<T> for [T] {
    fn get_by(&self, index: i32) -> Option<&T> {
        if index < 0 {
            return None;
        }

        let index = index as usize;
        self.get(index)
    }
}

pub trait TableExtensions<T> {
    fn get_by_p(&self, index: P) -> Option<&T>;
    fn is_at_p(&self, index: P, required: &T) -> bool;
}

impl<T: std::cmp::PartialEq> TableExtensions<T> for [Vec<T>] {
    fn get_by_p(&self, index: P) -> Option<&T> {
        self.get_by(index.y).and_then(|line| line.get_by(index.x))
    }

    fn is_at_p(&self, index: P, required: &T) -> bool {
        self.get_by_p(index).is_some_and(|value| value == required)
    }
}
