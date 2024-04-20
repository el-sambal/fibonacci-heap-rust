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
        unsafe { // my first `unsafe` ever! :) 20 april 2024
            if self.min.is_null() {
                (*node).left = node;
                (*node).right = node;
                self.min = node;
            } else {
                // put new element in root list
                // TODO

                if (*node).key < (*self.min).key {
                    self.min = node;
                }
            }
        }
        self.n += 1;
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
