use std::marker::PhantomData;

struct ListNode<T> {
    item: T,
    next: Option<Box<ListNode<T>>>,
}

pub struct LifoList<T> {
    head: Option<Box<ListNode<T>>>,
    size: usize,
}

impl<T> LifoList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }

    pub fn empty(&self) -> bool {
        self.size == 0
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, item: T) {
        self.head = Some(Box::new(ListNode {
            item,
            next: self.head.take(),
        }));
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(node) => {
                self.head = node.next;
                self.size -= 1;
                Some(node.item)
            }
        }
    }
}


// Drop implementation to clean up the list when it goes out of scope
impl<T> Drop for LifoList<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

// Iterator implementation for LifoList
pub struct LifoListIterator<T> {
    node: Option<Box<ListNode<T>>>,
}
 
impl<T> Iterator for LifoListIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.node.take() {
            None => None,
            Some(current_node) => {
                self.node = current_node.next; // transfer ownership of next node
                Some(current_node.item)
            }
        }
    }
}
 
impl<T> IntoIterator for LifoList<T> {
    type Item = T;
    type IntoIter = LifoListIterator<T>;
    
    fn into_iter(mut self) -> LifoListIterator<T> {
        LifoListIterator { node: self.head.take() }
    }
}

pub struct LifoListIter<'a, T> {
    next: Option<&'a ListNode<T>>,
}

impl<'a, T> Iterator for LifoListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next {
            None => None,
            Some(node) => {
                self.next = node.next.as_deref(); // getting reference to next node
                Some(&node.item)
            }
        }
    }
}

impl<T> LifoList<T> {
    pub fn iter(&self) -> LifoListIter<'_, T> {
        LifoListIter {
            next: self.head.as_deref(),
        }
    }
}

#[cfg(test)]
mod tests_lifo_list {
    use super::*;
    use rstest::{fixture, rstest};

    #[rstest]
    fn list_into_iter(lst_with_items: LifoList<i32>) {
        let lst = lst_with_items;

        let mut it = lst.into_iter();
        assert_eq!(it.next(), Some(3));
        assert_eq!(it.next(), Some(2));
        assert_eq!(it.next(), Some(1));
        assert_eq!(it.next(), None);
    }

    #[rstest]
    fn list_for_into_iter(lst_with_items: LifoList<i32>) {
        let mut items = vec![];
        for item in lst_with_items {
            items.push(item);
        }
        assert_eq!(items, vec![3, 2, 1]);
    }

    #[rstest]
    fn list_iter(lst_with_items: LifoList<i32>) {
        let lst = lst_with_items;

        let mut it = lst.iter();
        assert_eq!(it.next(), Some(&3));
        assert_eq!(it.next(), Some(&2));
        assert_eq!(it.next(), Some(&1));
        assert_eq!(it.next(), None);
    }

    // #[rstest]
    // fn list_iter_mut(mut lst_with_items: LifoList<i32>) {
    //     let mut it = lst_with_items.iter_mut();
    //     assert_eq!(it.next(), Some(&mut 3));
    //     assert_eq!(it.next(), Some(&mut 2));
    //     assert_eq!(it.next(), Some(&mut 1));
    //     assert_eq!(it.next(), None);
    // }

    #[fixture]
    fn lst_with_items() -> LifoList<i32> {
        let mut lst = LifoList::new();
        lst.push(1);
        lst.push(2);
        lst.push(3);
        lst
    }

    #[test]
    fn new_list_is_empty() {
        let lst = LifoList::<i32>::new();

        assert!(lst.empty());
    }

    #[test]
    fn new_list_size_is_zero() {
        let lst = LifoList::<i32>::new();

        assert_eq!(lst.size(), 0);
    }

    #[test]
    fn new_list_when_item_pushed_then_list_is_no_longer_empty() {
        let mut lst = LifoList::<i32>::new();
        assert!(lst.empty());

        lst.push(1);
        assert!(!lst.empty());
    }

    #[test]
    fn new_list_when_item_pushed_then_size_is_increased() {
        let mut lst = LifoList::<i32>::new();
        assert_eq!(lst.size(), 0);
        lst.push(1);
        assert_eq!(lst.size(), 1);
    }

    #[test]
    #[allow(non_snake_case)]
    fn pop_returns_items_in_LIFO_order() {
        // Arrange
        let mut lst = LifoList::<i32>::new();
        lst.push(1);
        lst.push(2);
        lst.push(3);

        // Act / Assert
        assert_eq!(lst.pop(), Some(3));
        assert_eq!(lst.pop(), Some(2));
        assert_eq!(lst.pop(), Some(1));
        assert_eq!(lst.pop(), None);
    }

    #[rstest]
    fn when_item_is_popped_size_is_decreased(lst_with_items: LifoList<i32>) {
        let mut lst = lst_with_items;
        assert_eq!(lst.size(), 3);

        lst.pop();
        assert_eq!(lst.size(), 2);
        lst.pop();
        assert_eq!(lst.size(), 1);
        lst.pop();
        assert_eq!(lst.size(), 0);
        lst.pop();
        assert_eq!(lst.size(), 0);
    }

    #[rstest]
    fn when_all_items_are_popped_then_list_is_empty(lst_with_items: LifoList<i32>) {
        let mut lst = lst_with_items;
        lst.pop();
        lst.pop();
        lst.pop();
        assert!(lst.empty());
    }

    #[rstest]
    fn list_stress_test() {
        let mut lst = LifoList::new();

        let size: usize = 1_000_000;
        for i in 0..size {
            lst.push(i);
        }
    }

    //     #[rstest]
    //     fn list_collect() {
    //         let lst: LifoList<_> = (1..=5).collect();

    //         assert_eq!(lst.iter().cmp(&vec![5, 4, 3, 2, 1]), Ordering::Equal);
    //     }
}

// #[cfg(test)]
// mod tests_lifo_list_display {
//     use rstest::rstest;
//     use crate::lifo_list::LifoList;

//     #[rstest]
//     fn empty_list() {
//         let lst: LifoList<i32> = LifoList::new();

//         let fmt_output = format!("{}", lst);
//         assert_eq!(fmt_output, "LifoList: []");
//     }

//     #[rstest]
//     fn list_with_items() {
//         let mut lst: LifoList<i32> = LifoList::new();
//         lst.push(1);
//         lst.push(2);
//         lst.push(3);

//         let fmt_output = format!("{}", lst);
//         assert_eq!(fmt_output, "LifoList: [3, -> [2, -> [1, None]]");
//     }
// }
