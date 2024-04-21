use std::{cell::RefCell, rc::Rc};

/// This is a min-Fibonacci heap.
pub struct FibonacciHeap<T> {
    /// The current number of nodes in the Fibonacci heap.
    n: usize,
    /// A pointer to the current minimal node in the Fibonacci heap. This is a null pointer if the
    /// heap is empty.
    min: *mut Node<T>,
}

pub struct NodePtr<T>(Rc<RefCell<NodePtrInternal<T>>>);

struct NodePtrInternal<T> {
    invalidated: bool,
    ptr: *mut Node<T>,
}

/// A node in the Fibonacci heap, containing the key, some pointers to other nodes and some
/// additional information.
struct Node<T> {
    /// The key of the node.
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
    /// The number of nodes in the child list of this node.
    degree: usize,
    /// A boolean flag which is true if and only if this node has lost a child node since the last time
    /// it was made the child of another node.
    mark: bool,
    outside_ref: Rc<RefCell<NodePtrInternal<T>>>,
}

impl<T> NodePtr<T> {
    pub fn delete(self) {
        println!(
            "todo: implement this! ptr: {:?} invalidated: {:?}",
            self.0.borrow().ptr,
            self.0.borrow().invalidated
        );
    }
}

impl<T: Ord> FibonacciHeap<T> {
    /// Construct a new, empty Fibonacci heap.
    ///
    /// Just like this:
    ///
    /// ```
    /// use fibonacci_heap_rust::FibonacciHeap;
    /// let mut heap: FibonacciHeap<String> = FibonacciHeap::new();
    /// assert!(heap.is_empty());
    /// ```
    pub const fn new() -> FibonacciHeap<T> {
        FibonacciHeap {
            n: 0,
            min: std::ptr::null_mut(),
        }
    }

    /// Checks whether the Fibonacci heap is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use fibonacci_heap_rust::FibonacciHeap;
    /// let mut heap: FibonacciHeap<String> = FibonacciHeap::new();
    /// assert!(heap.is_empty());
    /// heap.push("Heap is not empty anymore!".to_string());
    /// assert!(!heap.is_empty());
    /// let _ = heap.pop();
    /// assert!(heap.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.min.is_null()
    }

    /// Returns the number of elements currently in the heap.
    ///
    /// An example:
    ///
    /// ```
    /// use fibonacci_heap_rust::FibonacciHeap;
    /// let mut heap: FibonacciHeap<u32> = FibonacciHeap::new();
    /// assert!(heap.len() == 0);
    /// heap.push(5);
    /// assert!(heap.len() == 1);
    /// heap.push(42);
    /// assert!(heap.len() == 2);
    /// heap.push(2);
    /// assert!(heap.len() == 3);
    /// let _ = heap.pop();
    /// assert!(heap.len() == 2);
    /// let _ = heap.pop();
    /// assert!(heap.len() == 1);
    /// let _ = heap.pop();
    /// assert!(heap.len() == 0);
    /// let _ = heap.pop();
    /// assert!(heap.len() == 0);
    /// ```
    pub fn len(&self) -> usize {
        self.n
    }

    /// Produce a Fibonacci heap from melding two existing Fibonacci heaps. The two inputs are
    /// consumed.
    ///
    /// # Examples
    ///
    /// ```
    /// use fibonacci_heap_rust::FibonacciHeap;
    /// let mut heap1: FibonacciHeap<String> = FibonacciHeap::new();
    /// let mut heap2: FibonacciHeap<String> = FibonacciHeap::new();
    /// heap1.push("Hello".to_string());
    /// heap2.push("World".to_string());
    /// heap2.push("El-sambal".to_string());
    /// let mut heap: FibonacciHeap<String> = FibonacciHeap::from_meld(heap1,heap2);
    /// assert_eq!(heap.pop(), Some("El-sambal".to_string()));
    /// assert_eq!(heap.pop(), Some("Hello".to_string()));
    /// assert_eq!(heap.pop(), Some("World".to_string()));
    /// assert_eq!(heap.pop(), None);
    /// ```
    ///
    /// You can also meld empty heaps:
    ///
    /// ```
    /// use fibonacci_heap_rust::FibonacciHeap;
    /// let mut heap1: FibonacciHeap<String> = FibonacciHeap::from(
    ///     ["Hello".to_string(), "World".to_string()]
    /// );
    /// let mut heap2: FibonacciHeap<String> = FibonacciHeap::new();
    /// // heap2 is empty
    /// let mut heap3: FibonacciHeap<String> = FibonacciHeap::from_meld(heap1,heap2);
    /// assert_eq!(heap3.pop(), Some("Hello".to_string()));
    /// assert_eq!(heap3.pop(), Some("World".to_string()));
    /// assert_eq!(heap3.pop(), None);
    /// let mut heap4: FibonacciHeap<String> = FibonacciHeap::new();
    /// // heap3 and heap4 are both empty
    /// let mut heap5: FibonacciHeap<String> = FibonacciHeap::from_meld(heap3, heap4);
    /// assert!(heap5.is_empty());
    /// ```
    ///
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
        std::mem::forget(heap1);
        std::mem::forget(heap2);
        heap
    }

    /// Insert an element into the Fibonacci heap.
    ///
    /// A raw pointer to the node containing the inserted element is returned. You can pass this
    /// pointer into the `decrease_key` and `delete` methods. If you are not going to use these
    /// methods on the inserted element, you can discard the pointer outputted by this method.
    ///
    /// Just like this:
    ///
    /// ```
    /// use fibonacci_heap_rust::FibonacciHeap;
    /// let mut heap: FibonacciHeap<u32> = FibonacciHeap::new();
    /// assert!(heap.len() == 0);
    /// heap.push(5);
    /// assert!(heap.len() == 1);
    /// heap.push(42);
    /// assert!(heap.len() == 2);
    /// heap.push(2);
    /// assert!(heap.len() == 3);
    /// ```
    pub fn push(&mut self, item: T) -> NodePtr<T> {
        let node: *mut Node<T> = Box::into_raw(Box::new(Node {
            key: item,
            left: std::ptr::null_mut(),
            right: std::ptr::null_mut(),
            parent: std::ptr::null_mut(),
            child: std::ptr::null_mut(),
            degree: 0,
            mark: false,
            outside_ref: Rc::new(RefCell::new(NodePtrInternal {
                ptr: std::ptr::null_mut(),
                invalidated: false,
            })),
        }));
        unsafe {
            // my first `unsafe` ever! :) 20 april 2024
            (*node).outside_ref.borrow_mut().ptr = node;
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
            self.n += 1;
            NodePtr((*node).outside_ref.clone())
        }
    }

    /// Extracts the minimum element from the Fibonacci heap and returns it.
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
                    FibonacciHeap::remove_from_circular_list(popped);
                    self.min = (*popped).right;
                    self.consolidate();
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

    /// This method basically fixes up the Fibonacci heap (it is called by the `pop()` method) such
    /// that every root in the root list has a unique degree. This reduces the number of trees and
    /// that is good.
    unsafe fn consolidate(&mut self) {
        // if arr[i] = some node, then that node is a root with degree i
        let mut arr: Vec<*mut Node<T>> = vec![
                std::ptr::null_mut();
                (self.n as f64).log((1.0 + (5f64).sqrt()) / 2.0).floor() as usize + 1 // :-)
            ];

        // Make sure that each node in the root list has a unique degree
        let last = (*self.min).left;
        let mut node_it = last;
        let mut finished = false;
        while !finished {
            // Iterate over nodes in root list
            node_it = (*node_it).right;
            let mut x = node_it;
            if std::ptr::eq(x, last) {
                finished = true;
            }
            let mut d = (*x).degree;
            while !arr[d].is_null() {
                let mut y = arr[d];
                if (*x).key > (*y).key {
                    std::mem::swap(&mut x, &mut y);
                }

                // Make y a child of x
                if node_it == y {
                    // we were iterating over all nodes in root list, but the node we're
                    // currently at is now going to be moved out of the root list, so account for
                    // that
                    node_it = (*node_it).left;
                }
                FibonacciHeap::remove_from_circular_list(y);
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

    /// Removes an element from a circular doubly linked list. This function does no freeing whatsoever.
    /// The node pointed to by `elem` is not changed; its key and pointers stay intact.
    unsafe fn remove_from_circular_list(elem: *const Node<T>) {
        (*(*elem).right).left = (*elem).left;
        (*(*elem).left).right = (*elem).right;
    }
}

impl<T> Drop for FibonacciHeap<T> {
    fn drop(&mut self) {
        unsafe fn drop_recursive<T>(mut elem: *mut Node<T>) {
            if elem.is_null() {
                return;
            }
            (*(*elem).left).right = std::ptr::null_mut();
            loop {
                drop_recursive((*elem).child);
                if (*elem).right.is_null() {
                    let _ = Box::from_raw(elem);
                    break;
                }
                elem = (*elem).right;
                let _ = Box::from_raw((*elem).left);
            }
        }
        unsafe {
            drop_recursive(self.min);
        }
    }
}

impl<T: Ord, const N: usize> From<[T; N]> for FibonacciHeap<T> {
    /// Constructs a Fibonacci heap from an array of items.
    fn from(elems: [T; N]) -> Self {
        let mut heap = FibonacciHeap::new();
        for elem in elems {
            heap.push(elem);
        }
        heap
    }
}

impl<T: Ord> From<Vec<T>> for FibonacciHeap<T> {
    /// Constructs a Fibonacci heap from a `Vec` of items.
    fn from(elems: Vec<T>) -> Self {
        let mut heap = FibonacciHeap::new();
        for elem in elems {
            heap.push(elem);
        }
        heap
    }
}

impl<T: Ord> From<FibonacciHeap<T>> for Vec<T> {
    /// Constructs a `Vec` of items from a `FibonacciHeap`. The items in the `Vec` are sorted in
    /// increasing order (minimal element first). The Fibonacci heap is consumed.
    fn from(mut heap: FibonacciHeap<T>) -> Vec<T> {
        let mut res = vec![];
        while let Some(popped) = heap.pop() {
            res.push(popped);
        }
        res
    }
}

impl<T: Ord> Default for FibonacciHeap<T> {
    /// Creates an empty `FibonacciHeap<T>`.
    fn default() -> FibonacciHeap<T> {
        FibonacciHeap::new()
    }
}

#[cfg(test)]
mod tests;
