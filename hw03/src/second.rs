#[derive(Debug)]
pub struct BST<T> {
    root: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T: Ord> BST<T> {
    pub fn new() -> Self {
        BST {root: None}
    }

    pub fn insert(&mut self, elem: T) -> bool {
        let mut curr = &mut self.root;

        loop {
            let tmp = curr;
            if let Some(ref mut node) = *tmp {
                if node.elem == elem {
                    return false;
                } else if node.elem > elem {
                    curr = &mut node.left;
                } else {
                    curr = &mut node.right;
                }
            } else {
                curr = tmp;
                break;
            }
        }

        *curr = Some(Box::new( Node {
            elem: elem,
            left: None,
            right: None,
        }));
        true
    }

    pub fn search(&self, elem: T) -> bool {
        let mut curr = &self.root;
        while let Some(ref node) = *curr {
            if node.elem == elem {
                return true;
            } else if node.elem > elem {
                curr = &node.left;
            } else {
                curr = &node.right;
            }
        }
        false
    }

}

pub struct IntoIter<T> {
    root: Link<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.root.take().map(|node| {
            let node = *node;
            self.root = node.right;
            node.elem
        })
    }
}

impl<T> IntoIterator for BST<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {root: self.root}
    }
}

pub struct Iter<'a, T:'a> {
    root: &'a Link<T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.root.as_ref().map(|node| {
            self.root = &node.right;
            &node.elem
        })
    }
}

impl<'a, T> IntoIterator for &'a BST<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter { root: &self.root  }
    }
}

pub struct IterMut<'a, T: 'a> {
    root: Option<&'a mut Node<T>>,
}

impl<'a, T: 'a> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.root.take().map(|node| {
            self.root = node.right.as_mut().map(|node| &mut **node);
            &mut node.elem
        })
    }
}

impl<'a, T> IntoIterator for &'a mut BST<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut { root: self.root.as_mut().map(|node| &mut **node) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut bst = BST::new();
        bst.insert(10);
        bst.insert(1);
        bst.insert(14);
        bst.insert(14);

        println!("{:#?}", bst);

        assert_eq!(false, bst.search(4));
        assert_eq!(true, bst.search(14));
        assert_eq!(true, bst.search(10));
        assert_eq!(true, bst.search(1));

    }

    #[test]
    fn test_into_iter() {
        let mut bst = BST::new();
        for i in 0.. 5 {
            bst.insert(i);
        }

    }

    #[test]
    fn test_iter() {
        let mut bst = BST::new();
        for i in 0.. 5 {
            bst.insert(i);
        }

        for i in &bst {
            println!("{:#?}", i);
        }

        for i in bst {
            println!("{:#?}", i);
        }
    }

    #[test]
    fn test_iter_mut() {
        let mut bst = BST::new();
        for i in 0..100 {
            bst.insert(i);
        }

        for (i, r) in (&mut bst).into_iter().enumerate() {
            assert_eq!(i, *r);
            *r += 100;
        }

        for (i, r) in (&mut bst).into_iter().enumerate() {
            assert_eq!(i + 100 , *r);
        }

    }


}

