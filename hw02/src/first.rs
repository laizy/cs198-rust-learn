#[derive(Debug)]
pub struct BST {
    root: Link,
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
struct Node {
    elem: i32,
    left: Link,
    right: Link,
}


impl BST {
    pub fn new() -> Self {
        BST {root: Link::Empty}
    }

    pub fn insert(&mut self, elem: i32) -> bool {
        let mut curr = &mut self.root;

        loop {
            let tmp = curr;
            if let Link::More(ref mut node) = *tmp {
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

        *curr = Link::More(Box::new( Node {
            elem: elem,
            left: Link::Empty,
            right: Link::Empty,
        }));
        true
    }

    pub fn search(&self, elem: i32) -> bool {
        let mut curr = &self.root;
        while let Link::More(ref node) = *curr {
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
    fn test_insert2() {
        let mut bst = BST::new();
        for i in 0.. 5 {
            bst.insert(i);
        }

        println!("{:#?}", bst);
    }

}

