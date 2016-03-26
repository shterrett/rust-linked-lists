use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn append(&self, elem: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                elem: elem,
                next: self.head.clone()
            }))
        }
    }

    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone())
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.append(5);
        assert_eq!(list.head(), Some(&5));

        let list = list.append(7);
        assert_eq!(list.head(), Some(&7));

        let list = list.append(9);
        assert_eq!(list.head(), Some(&9));

        let list = list.tail();
        assert_eq!(list.head(), Some(&7));

        let list = list.tail();
        assert_eq!(list.head(), Some(&5));

        let list = list.tail();
        assert_eq!(list.head(), None);
    }
}