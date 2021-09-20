use std::slice::Iter;

pub enum Tree<T> {
    Leaf(T),
    Children(Vec<Tree<T>>),
}

// https://www.reddit.com/r/rust/comments/fsbqwp/how_to_iterate_trees_nicely
impl<T> Tree<T> {
    fn iter(&self) -> impl Iterator<Item = &T> {

        fn traverse_depth<U>(start: &Tree<U>, stack: &mut Vec<Iter<Tree<U>>>) -> Option<U> {
            todo!()
        }

        std::iter::from_fn() {
            todo!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree() {
        let tree = Tree::Children(vec![
            Tree::Leaf(vec![1u32, 2, 3, 4]),
            Tree::Leaf(vec![5, 6, 7, 8]),
            Tree::Children(vec![
                Tree::Leaf(vec![9, 10]),
                Tree::Leaf(vec![11, 12]),
                Tree::Children(vec![Tree::Leaf(vec![13, 14]), Tree::Leaf(vec![15, 16])]),
                Tree::Leaf(vec![17, 18]),
            ]),
            Tree::Leaf(vec![19, 20]),
        ]);
        let res: Vec<_> = tree.iter().copied().collect();
        assert_eq!(&res, &(1..=20).collect::<Vec<u32>>());
    }
}
