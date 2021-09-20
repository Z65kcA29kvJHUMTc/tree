use std::slice::Iter;

pub enum Tree<T> {
    Leaf(T),
    Children(Vec<Tree<T>>),
}

// https://www.reddit.com/r/rust/comments/fsbqwp/how_to_iterate_trees_nicely
impl<T> Tree<T> {
    fn iter(&self) -> impl Iterator<Item = &T> {
        fn traverse_depth<'a, U>(
            start: &'a Tree<U>,
            stack: &mut Vec<Iter<'a, Tree<U>>>,
        ) -> Option<&'a U> {
            let mut node = start;
            loop {
                match *node {
                    Tree::Leaf(ref item) => break Some(item),
                    Tree::Children(ref children) => stack.push(children.iter()),
                }
                node = stack.last_mut().unwrap().next()?;
            }
        }

        let mut stack = Vec::new();
        let mut leaf = traverse_depth(self, &mut stack);

        std::iter::from_fn({
            move || loop {
                if let Some(next) = leaf {
                    break Some(next);
                }
                if let Some(next) = stack.last_mut()?.next() {
                    leaf = traverse_depth(next, &mut stack);
                } else {
                    stack.pop();
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree() {
        let tree = Tree::Children(vec![
            Tree::Leaf(1),
            Tree::Leaf(2),
            Tree::Children(vec![
                Tree::Leaf(3),
                Tree::Leaf(4),
                Tree::Children(vec![Tree::Leaf(5), Tree::Leaf(6)]),
                Tree::Leaf(7),
            ]),
            Tree::Leaf(8),
        ]);
        let res: Vec<_> = tree.iter().copied().collect();
        assert_eq!(&res, &(1..=8).collect::<Vec<u32>>());
    }
}
