/// This is a min-Fibonacci heap.
pub struct FibonacciHeap<T> {
    /// The current number of nodes in the Fibonacci heap
    n: usize,
    /// A pointer to the current minimal node in the Fibonacci heap. This is a null pointer if the
    /// heap is empty.
    min: *mut Node<T>,
}

impl<T: Ord> FibonacciHeap<T> {
    pub const fn new() -> FibonacciHeap<T> {
        FibonacciHeap {
            n: 0,
            min: std::ptr::null_mut(),
        }
    }

    pub fn is_empty(&self) -> bool {
        !self.min.is_null()
    }

    pub fn from_meld(heap1: FibonacciHeap<T>, heap2: FibonacciHeap<T>) -> FibonacciHeap<T> {
        let mut heap = FibonacciHeap::<T>::new();
        if heap1.is_empty() {
            return heap2;
        } else if heap2.is_empty() {
            return heap1;
        }
        unsafe {
            FibonacciHeap::concatenate_circular_lists(heap1.min, heap2.min);

            heap.min = if (*heap1.min).key < (*heap2.min).key {
                heap1.min
            } else {
                heap2.min
            };
        }
        heap.n = heap1.n + heap2.n;
        heap
    }

    pub fn pop(&mut self) -> Option<T> {
        let popped = self.min;
        if !popped.is_null() {
            unsafe {
                let mut child = (*popped).child;
                if !child.is_null() {
                    while !(*child).parent.is_null() {
                        (*child).parent = std::ptr::null_mut();
                        child = (*child).right;
                    }
                }
                FibonacciHeap::concatenate_circular_lists(child, popped);
                if (*popped).right != popped {
                    FibonacciHeap::remove_from_circular_list(popped, (*popped).right);
                    self.min = (*popped).right;
                    Self::confirm_integrity(self.min);
                    self.consolidate();
                    Self::confirm_integrity(self.min);
                } else {
                    self.min = std::ptr::null_mut();
                }
            }
            self.n -= 1;
            unsafe { Some(Box::from_raw(popped).key) }
        } else {
            None
        }
    }

    pub fn push(&mut self, item: T) {
        let node: *mut Node<T> = Box::into_raw(Box::new(Node {
            key: item,
            left: std::ptr::null_mut(),
            right: std::ptr::null_mut(),
            parent: std::ptr::null_mut(),
            child: std::ptr::null_mut(),
            degree: 0,
            mark: false,
        }));
        unsafe {
            // my first `unsafe` ever! :) 20 april 2024
            if self.min.is_null() {
                (*node).left = node;
                (*node).right = node;
                self.min = node;
            } else {
                FibonacciHeap::add_node_to_nonempty_circular_list(node, self.min);

                if (*node).key < (*self.min).key {
                    self.min = node;
                }
            }
        }
        self.n += 1;
    }

    /// Adds a node to a circular doubly linked list. Both inputs must not be null pointers.
    unsafe fn add_node_to_nonempty_circular_list(new_item: *mut Node<T>, list: *mut Node<T>) {
        (*new_item).right = (*list).right;
        (*new_item).left = list;
        (*(*list).right).left = new_item;
        (*list).right = new_item;
    }

    /// Concatenates two circular doubly linked lists.
    unsafe fn concatenate_circular_lists(list1: *mut Node<T>, list2: *mut Node<T>) {
        if list1.is_null() || list2.is_null() {
            return;
        }
        let prev_list1_right = (*list1).right;
        let prev_list2_left = (*list2).left;
        (*list1).right = list2;
        (*list2).left = list1;
        (*prev_list1_right).left = prev_list2_left;
        (*prev_list2_left).right = prev_list1_right;
    }

    /// Removes an element from a circular doubly linked list. The specified element must be a
    /// member of this list, otherwise it is UB. `elem` and `list` should not be equal, otherwise
    /// it is UB. This function does no freeing whatsoever. The node pointed to by `elem` is not
    /// changed; its key and pointers stay intact.
    unsafe fn remove_from_circular_list(elem: *const Node<T>, list: *mut Node<T>) {
        debug_assert!(!std::ptr::eq(elem, list));
        debug_assert!(!std::ptr::eq((*elem).right, elem));
        (*(*elem).right).left = (*elem).left;
        (*(*elem).left).right = (*elem).right;
    }

    unsafe fn consolidate(&mut self) {
        // if arr[i] = some node, then that node is a root with degree i
        let mut arr: Vec<*mut Node<T>> = vec![
                std::ptr::null_mut();
                (self.n as f64).log((1.0 + (5f64).sqrt()) / 2.0).floor() as usize + 1 // :-)
            ];

        // make sure that each node in the root list has a unique degree
        let last = (*self.min).left;
        let mut node_it = last;
        let mut finished = false;
        while !finished {
            // iterate over nodes in root list
            node_it = (*node_it).right;
            FibonacciHeap::confirm_integrity(node_it);
            let mut x = node_it;
            if std::ptr::eq(x, last) {
                finished = true;
            }
            let mut d = (*x).degree;
            while !arr[d].is_null() {
                let mut y = arr[d];
                debug_assert!(!std::ptr::eq(x, y));
                if (*x).key > (*y).key {
                    std::mem::swap(&mut x, &mut y);
                }

                // make y a child of x
                Self::confirm_integrity(self.min);
                FibonacciHeap::remove_from_circular_list(y, x);
                (*x).degree += 1;
                if !(*x).child.is_null() {
                    Self::add_node_to_nonempty_circular_list(y, (*x).child);
                } else {
                    (*y).left = y;
                    (*y).right = y;
                    (*x).child = y;
                }
                (*y).mark = false;
                (*y).parent = x;
                Self::confirm_integrity(self.min);

                arr[d] = std::ptr::null_mut();
                d += 1;
            }
            arr[d] = x;
        }

        self.min = std::ptr::null_mut();
        // root list is intact, but we need to find out who is the new `min`
        let mut min: *mut Node<T> = std::ptr::null_mut();
        for node in arr {
            if !node.is_null() && (min.is_null() || (*node).key < (*min).key) {
                min = node;
            }
        }
        self.min = min;
    }

    /// For debugging
    unsafe fn confirm_integrity(list: *mut Node<T>) {
        let mut node_it = (*list).right;
        loop {
            assert!((*(*node_it).right).left == node_it);
            assert!((*(*node_it).left).right == node_it);
            assert!((*(*node_it).left).parent == (*node_it).parent);
            assert!((*node_it).child.is_null() || (*(*node_it).child).parent == node_it);
            if !(*node_it).child.is_null() {
                Self::confirm_integrity((*node_it).child);
            }

            node_it = (*node_it).right;
            if node_it == list {
                break;
            }
        }
    }
}

impl<T> Drop for FibonacciHeap<T> {
    fn drop(&mut self) {
        println!("Dropping still has to be implemented. I\'m gonna leak memory now!");
    }
}

struct Node<T> {
    /// The key of the node
    key: T,
    /// The left neighbor of this node in the circular doubly linked list.
    ///
    /// If this node is the only element in the circular doubly linked list, then this is a pointer
    /// to the node itself.
    left: *mut Node<T>,
    /// The right neighbor of this node in the circular doubly linked list.
    ///
    /// If this node is the only element in the circular doubly linked list, then this is a pointer
    /// to the node itself.
    right: *mut Node<T>,
    /// The parent of this node. This is a null pointer if the node has no parent.
    parent: *mut Node<T>,
    /// A pointer to *any* child of this node. This is a null pointer if the node has no children.
    child: *mut Node<T>,
    /// The number of nodes in the child list of this node
    degree: usize,
    /// A boolean flag which is true if and only if this node has lost a child node since the last time
    /// it was made the child of another node
    mark: bool,
}

#[cfg(test)]
mod tests {
    use crate::FibonacciHeap;

    #[test]
    fn test_new() {
        let _: FibonacciHeap<String> = FibonacciHeap::new();
        let _: FibonacciHeap<i64> = FibonacciHeap::new();
        let _: FibonacciHeap<u8> = FibonacciHeap::new();
    }

    #[test]
    fn test_push_and_pop_one_element() {
        let mut fh: FibonacciHeap<usize> = FibonacciHeap::new();
        fh.push(1);
        assert_eq!(fh.pop(), Some(1));
    }

    #[test]
    fn test_push_and_pop_two_elements_1() {
        let mut fh: FibonacciHeap<usize> = FibonacciHeap::new();
        fh.push(1);
        fh.push(2);
        assert_eq!(fh.pop(), Some(1));
        assert_eq!(fh.pop(), Some(2));
        assert_eq!(fh.pop(), None);
    }

    #[test]
    fn test_push_and_pop_two_elements_2() {
        let mut fh: FibonacciHeap<usize> = FibonacciHeap::new();
        fh.push(2);
        fh.push(1);
        assert_eq!(fh.pop(), Some(1));
        assert_eq!(fh.pop(), Some(2));
        assert_eq!(fh.pop(), None);
    }

    #[test]
    fn test_push_and_pop_three_elements_1() {
        let mut fh: FibonacciHeap<usize> = FibonacciHeap::new();
        fh.push(2);
        fh.push(1);
        fh.push(3);
        assert_eq!(fh.pop(), Some(1));
        assert_eq!(fh.pop(), Some(2));
        assert_eq!(fh.pop(), Some(3));
        assert_eq!(fh.pop(), None);
    }

    #[test]
    fn test_push_and_pop_three_elements_2() {
        let mut fh: FibonacciHeap<usize> = FibonacciHeap::new();
        fh.push(1);
        fh.push(3);
        fh.push(2);
        assert_eq!(fh.pop(), Some(1));
        assert_eq!(fh.pop(), Some(2));
        assert_eq!(fh.pop(), Some(3));
        assert_eq!(fh.pop(), None);
    }

    #[test]
    fn test_push_and_pop_three_elements_3() {
        let mut fh: FibonacciHeap<usize> = FibonacciHeap::new();
        fh.push(1);
        fh.push(2);
        fh.push(3);
        assert_eq!(fh.pop(), Some(1));
        assert_eq!(fh.pop(), Some(2));
        assert_eq!(fh.pop(), Some(3));
        assert_eq!(fh.pop(), None);
    }

    #[test]
    fn test_push_many_then_pop_many() {
        let mut fh: FibonacciHeap<i32> = FibonacciHeap::new();
        fh.push(42);
        fh.push(-42);
        fh.push(-137);
        fh.push(137);
        for i in 0..1000 {
            fh.push((i * i * i) % 3000);
        }
        let mut prev = i32::max_value();
        while let Some(popped) = fh.pop() {
            println!("popped!");
            assert!(popped >= prev);
            prev = popped;
        }
    }
}
