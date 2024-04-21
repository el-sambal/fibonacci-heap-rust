# Fibonacci heap

This is a Fibonacci (min)heap implementation, mainly inspired by the amazing 'CLRS' book (*Introduction to Algorithms*, by Thomas Cormen et al.). Fibonacci heaps are very nice data structures, because the `push()`, `peek_minimum()`, `from_meld()` and `decrease_key()` all have amortized $O(1)$ time complexity, and the `pop()` and `delete_node()` operations have amortized $O(\log n)$ time complexity.

Fibonacci heaps are also very complex data structures: each node contains *four* pointers to nodes. The data structure uses circular doubly linked lists under the hood for nodes that are on the same level (which is why each node has a `left` and `right` pointer), and each node additionally has a `parent` and `child` pointer.

Obviously, this doesn't play nice with Rust's ownership model, which is where the infamous `unsafe` comes to the rescue! I wanted to learn `unsafe` Rust, and this project has been a nice introduction it.