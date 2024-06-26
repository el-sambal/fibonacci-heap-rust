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
    let mut input: Vec<i32> = vec![];
    let mut output: Vec<i32> = vec![];
    for i in 0..1000 {
        fh.push((i * i * i) % 3000);
        input.push((i * i * i) % 3000);
    }
    let mut prev = i32::min_value();
    while let Some(popped) = fh.pop() {
        assert!(popped >= prev);
        prev = popped;
        output.push(popped);
    }
    input.sort();
    output.sort();
    assert!(input == output);
}

#[test]
fn test_push_many_then_pop_many_with_many_duplicates() {
    let mut fh: FibonacciHeap<i32> = FibonacciHeap::new();
    let mut input: Vec<i32> = vec![];
    let mut output: Vec<i32> = vec![];
    for i in 0..1000 {
        fh.push((i * i * i) % 300);
        input.push((i * i * i) % 300);
    }
    let mut prev = i32::min_value();
    while let Some(popped) = fh.pop() {
        assert!(popped >= prev);
        prev = popped;
        output.push(popped);
    }
    input.sort();
    output.sort();
    assert!(input == output);
}

#[test]
fn test_push_many_then_pop_many_with_very_many_duplicates() {
    let mut fh: FibonacciHeap<i32> = FibonacciHeap::new();
    let mut input: Vec<i32> = vec![];
    let mut output: Vec<i32> = vec![];
    for i in 0..1000 {
        fh.push((i * i * i) % 3);
        input.push((i * i * i) % 3);
    }
    let mut prev = i32::min_value();
    while let Some(popped) = fh.pop() {
        assert!(popped >= prev);
        prev = popped;
        output.push(popped);
    }
    input.sort();
    output.sort();
    assert!(input == output);
}

#[test]
#[cfg(not(miri))]
fn test_push_very_many_then_pop_very_many() {
    let mut fh: FibonacciHeap<i32> = FibonacciHeap::new();
    let mut input: Vec<i32> = vec![];
    let mut output: Vec<i32> = vec![];
    for i in 0..10000 {
        fh.push((i * i) % 2000);
        input.push((i * i) % 2000);
    }
    let mut prev = i32::min_value();
    while let Some(popped) = fh.pop() {
        assert!(popped >= prev);
        output.push(popped);
        prev = popped;
    }
    input.sort();
    output.sort();
    assert!(input == output);
}

#[test]
#[cfg(not(miri))]
fn test_push_very_many_then_pop_very_many_with_much_overlap() {
    let mut fh: FibonacciHeap<i32> = FibonacciHeap::new();
    let mut input: Vec<i32> = vec![];
    let mut output: Vec<i32> = vec![];
    for i in 0..10000 {
        fh.push((i * i) % 20);
        input.push((i * i) % 20);
    }
    let mut prev = i32::min_value();
    while let Some(popped) = fh.pop() {
        assert!(popped >= prev);
        output.push(popped);
        prev = popped;
    }
    input.sort();
    output.sort();
    assert!(input == output);
}

#[test]
#[cfg(not(miri))]
fn test_push_1000000_then_pop() {
    let mut fh: FibonacciHeap<i64> = FibonacciHeap::new();
    let mut input: Vec<i64> = vec![];
    let mut output: Vec<i64> = vec![];
    for i in 0..1000000 {
        fh.push((i * i) % 700000);
        input.push((i * i) % 700000);
    }
    let mut prev = i64::min_value();
    while let Some(popped) = fh.pop() {
        assert!(popped >= prev);
        output.push(popped);
        prev = popped;
    }
    input.sort();
    output.sort();
    assert!(input == output);
}

#[test]
#[cfg(not(miri))]
fn test_push_1000000_then_pop_with_much_overlap() {
    let mut fh: FibonacciHeap<i64> = FibonacciHeap::new();
    let mut input: Vec<i64> = vec![];
    let mut output: Vec<i64> = vec![];
    for i in 0..1000000 {
        fh.push((i * i) % 7000);
        input.push((i * i) % 7000);
    }
    let mut prev = i64::min_value();
    while let Some(popped) = fh.pop() {
        assert!(popped >= prev);
        output.push(popped);
        prev = popped;
    }
    input.sort();
    output.sort();
    assert!(input == output);
}

#[test]
fn test_push_heap_allocated_obj_for_drop_1() {
    let mut fh: FibonacciHeap<String> = FibonacciHeap::new();
    fh.push("Hello".to_string());
    fh.push("World".to_string());
    fh.push(",".to_string());
    fh.push("This".to_string());
    fh.push("is".to_string());
    fh.push("a".to_string());
    fh.push("test".to_string());
    fh.push("to".to_string());
    fh.push("make".to_string());
    fh.push("sure".to_string());
    fh.push("that".to_string());
    fh.push("deallocating".to_string());
    fh.push("the".to_string());
    fh.push("Fibonacci".to_string());
    fh.push("heap".to_string());
    fh.push("works".to_string());
}

#[test]
fn test_push_heap_allocated_obj_for_drop_2() {
    let mut fh: FibonacciHeap<String> = FibonacciHeap::new();
    fh.push("Hello".to_string());
}

#[test]
fn test_push_heap_allocated_obj_for_drop_3() {
    let mut fh: FibonacciHeap<String> = FibonacciHeap::new();
    fh.push("Hello".to_string());
    fh.push("Hello".to_string());
}

#[test]
fn test_push_heap_allocated_obj_for_drop_4() {
    let mut fh: FibonacciHeap<String> = FibonacciHeap::new();
    fh.push("Hello".to_string());
    fh.push("Hello".to_string());
    fh.push("Hello1".to_string());
}

#[test]
fn test_push_heap_allocated_obj_for_drop_5() {
    let mut fh: FibonacciHeap<String> = FibonacciHeap::new();
    fh.push("Hello".to_string());
    fh.push("World".to_string());
    fh.push(",".to_string());
    fh.push("This".to_string());
    fh.push("is".to_string());
    fh.push("a".to_string());
    fh.push("test".to_string());
    fh.push("to".to_string());
    fh.push("make".to_string());
    fh.push("sure".to_string());
    fh.push("that".to_string());
    fh.push("deallocating".to_string());
    fh.push("the".to_string());
    fh.push("Fibonacci".to_string());
    fh.push("heap".to_string());
    fh.push("works".to_string());
    assert_eq!(fh.pop(), Some(",".to_string()));
    assert_eq!(fh.pop(), Some("Fibonacci".to_string()));
    assert_eq!(fh.pop(), Some("Hello".to_string()));
    assert_eq!(fh.pop(), Some("This".to_string()));
    assert_eq!(fh.pop(), Some("World".to_string()));
    assert_eq!(fh.pop(), Some("a".to_string()));
}

#[test]
fn test_heap_meld_1() {
    let mut fh1: FibonacciHeap<String> = FibonacciHeap::new();
    let mut fh2: FibonacciHeap<String> = FibonacciHeap::new();
    fh1.push("Hello".to_string());
    fh2.push("World".to_string());
    fh2.push(",".to_string());
    fh2.push("This".to_string());
    fh2.push("is".to_string());
    fh2.push("a".to_string());
    fh1.push("test".to_string());
    fh1.push("to".to_string());
    fh1.push("make".to_string());
    fh1.push("sure".to_string());
    fh1.push("that".to_string());
    fh1.push("melding".to_string());
    fh1.push("the".to_string());
    fh2.push("Fibonacci".to_string());
    fh2.push("heap".to_string());
    fh1.push("works".to_string());
    let mut fh: FibonacciHeap<String> = FibonacciHeap::from_meld(fh1, fh2);
    assert_eq!(fh.pop(), Some(",".to_string()));
    assert_eq!(fh.pop(), Some("Fibonacci".to_string()));
    assert_eq!(fh.pop(), Some("Hello".to_string()));
    assert_eq!(fh.pop(), Some("This".to_string()));
    assert_eq!(fh.pop(), Some("World".to_string()));
    assert_eq!(fh.pop(), Some("a".to_string()));
    assert_eq!(fh.pop(), Some("heap".to_string()));
    assert_eq!(fh.pop(), Some("is".to_string()));
    assert_eq!(fh.pop(), Some("make".to_string()));
    assert_eq!(fh.pop(), Some("melding".to_string()));
    assert_eq!(fh.pop(), Some("sure".to_string()));
    assert_eq!(fh.pop(), Some("test".to_string()));
    assert_eq!(fh.pop(), Some("that".to_string()));
    assert_eq!(fh.pop(), Some("the".to_string()));
    assert_eq!(fh.pop(), Some("to".to_string()));
    assert_eq!(fh.pop(), Some("works".to_string()));
    assert_eq!(fh.pop(), None);
}

#[test]
fn test_heap_meld_2() {
    let mut fh1: FibonacciHeap<String> = FibonacciHeap::new();
    let mut fh2: FibonacciHeap<String> = FibonacciHeap::new();
    fh1.push("Aloha".to_string());
    fh2.push("Mundo".to_string());
    let mut fh: FibonacciHeap<String> = FibonacciHeap::from_meld(fh1, fh2);
    assert_eq!(fh.pop(), Some("Aloha".to_string()));
    assert_eq!(fh.pop(), Some("Mundo".to_string()));
    assert_eq!(fh.pop(), None);
}

#[test]
fn test_from_array() {
    let fh1: FibonacciHeap<String> = FibonacciHeap::from([
        "Hello".to_string(),
        "test".to_string(),
        "to".to_string(),
        "make".to_string(),
        "sure".to_string(),
        "that".to_string(),
        "melding".to_string(),
        "works".to_string(),
        "the".to_string(),
    ]);
    let fh2: FibonacciHeap<String> = FibonacciHeap::from([
        "World".to_string(),
        ",".to_string(),
        "This".to_string(),
        "is".to_string(),
        "a".to_string(),
        "Fibonacci".to_string(),
        "heap".to_string(),
    ]);
    let mut fh: FibonacciHeap<String> = FibonacciHeap::from_meld(fh1, fh2);
    assert_eq!(fh.pop(), Some(",".to_string()));
    assert_eq!(fh.pop(), Some("Fibonacci".to_string()));
    assert_eq!(fh.pop(), Some("Hello".to_string()));
    assert_eq!(fh.pop(), Some("This".to_string()));
    assert_eq!(fh.pop(), Some("World".to_string()));
    assert_eq!(fh.pop(), Some("a".to_string()));
    assert_eq!(fh.pop(), Some("heap".to_string()));
    assert_eq!(fh.pop(), Some("is".to_string()));
    assert_eq!(fh.pop(), Some("make".to_string()));
    assert_eq!(fh.pop(), Some("melding".to_string()));
    assert_eq!(fh.pop(), Some("sure".to_string()));
    assert_eq!(fh.pop(), Some("test".to_string()));
    assert_eq!(fh.pop(), Some("that".to_string()));
    assert_eq!(fh.pop(), Some("the".to_string()));
    assert_eq!(fh.pop(), Some("to".to_string()));
    assert_eq!(fh.pop(), Some("works".to_string()));
    assert_eq!(fh.pop(), None);
}

#[test]
fn test_from_vec() {
    let fh1: FibonacciHeap<String> = FibonacciHeap::from(vec![
        "Hello".to_string(),
        "test".to_string(),
        "to".to_string(),
        "make".to_string(),
        "sure".to_string(),
        "that".to_string(),
        "melding".to_string(),
        "works".to_string(),
        "the".to_string(),
    ]);
    let fh2: FibonacciHeap<String> = FibonacciHeap::from(vec![
        "World".to_string(),
        ",".to_string(),
        "This".to_string(),
        "is".to_string(),
        "a".to_string(),
        "Fibonacci".to_string(),
        "heap".to_string(),
    ]);
    let mut fh: FibonacciHeap<String> = FibonacciHeap::from_meld(fh1, fh2);
    assert_eq!(fh.pop(), Some(",".to_string()));
    assert_eq!(fh.pop(), Some("Fibonacci".to_string()));
    assert_eq!(fh.pop(), Some("Hello".to_string()));
    assert_eq!(fh.pop(), Some("This".to_string()));
    assert_eq!(fh.pop(), Some("World".to_string()));
    assert_eq!(fh.pop(), Some("a".to_string()));
    assert_eq!(fh.pop(), Some("heap".to_string()));
    assert_eq!(fh.pop(), Some("is".to_string()));
    assert_eq!(fh.pop(), Some("make".to_string()));
    assert_eq!(fh.pop(), Some("melding".to_string()));
    assert_eq!(fh.pop(), Some("sure".to_string()));
    assert_eq!(fh.pop(), Some("test".to_string()));
    assert_eq!(fh.pop(), Some("that".to_string()));
    assert_eq!(fh.pop(), Some("the".to_string()));
    assert_eq!(fh.pop(), Some("to".to_string()));
    assert_eq!(fh.pop(), Some("works".to_string()));
    assert_eq!(fh.pop(), None);
}
#[test]
fn test_from_to_vec() {
    let fh1: FibonacciHeap<String> = FibonacciHeap::from(vec![
        "Hello".to_string(),
        "test".to_string(),
        "to".to_string(),
        "make".to_string(),
        "sure".to_string(),
        "that".to_string(),
        "melding".to_string(),
        "works".to_string(),
        "the".to_string(),
    ]);
    let fh2: FibonacciHeap<String> = FibonacciHeap::from(vec![
        "World".to_string(),
        ",".to_string(),
        "This".to_string(),
        "is".to_string(),
        "a".to_string(),
        "Fibonacci".to_string(),
        "heap".to_string(),
    ]);
    let fh = Vec::from(FibonacciHeap::from_meld(fh1, fh2));
    let should_be = vec![
        ",".to_string(),
        "Fibonacci".to_string(),
        "Hello".to_string(),
        "This".to_string(),
        "World".to_string(),
        "a".to_string(),
        "heap".to_string(),
        "is".to_string(),
        "make".to_string(),
        "melding".to_string(),
        "sure".to_string(),
        "test".to_string(),
        "that".to_string(),
        "the".to_string(),
        "to".to_string(),
        "works".to_string(),
    ];
    assert_eq!(fh, should_be);
}
#[test]
fn test_default() {
    let fh1: FibonacciHeap<String> = Default::default();
    assert!(fh1.is_empty());
}

#[test]
fn test_decrease_key_1() {
    let mut fh: FibonacciHeap<String> = FibonacciHeap::new();
    fh.push("a1".to_string());
    fh.push("a2".to_string());
    fh.push("a3".to_string());
    let a4_ptr = fh.push("a4".to_string());
    fh.push("a5".to_string());
    fh.push("a6".to_string());
    fh.push("a7".to_string());
    assert_eq!(fh.pop(), Some("a1".to_string()));
    assert_eq!(fh.pop(), Some("a2".to_string()));
    fh.decrease_key(&a4_ptr, "a0".to_string());
    assert_eq!(fh.pop(), Some("a0".to_string()));
    assert_eq!(fh.pop(), Some("a3".to_string()));
    assert_eq!(fh.pop(), Some("a5".to_string()));
    assert_eq!(fh.pop(), Some("a6".to_string()));
    assert_eq!(fh.pop(), Some("a7".to_string()));
    assert_eq!(fh.pop(), None);
}

#[test]
fn test_decrease_key_2() {
    let mut fh: FibonacciHeap<String> = FibonacciHeap::new();
    fh.push("a1".to_string());
    fh.push("a2".to_string());
    fh.push("a3".to_string());
    let a4_ptr = fh.push("a4".to_string());
    fh.push("a5".to_string());
    fh.push("a6".to_string());
    fh.push("a7".to_string());
    assert_eq!(fh.pop(), Some("a1".to_string()));
    assert_eq!(fh.pop(), Some("a2".to_string()));
    fh.decrease_key(&a4_ptr, "a8".to_string()); // new key > old key: nothing happens
    assert_eq!(fh.pop(), Some("a3".to_string()));
    assert_eq!(fh.pop(), Some("a4".to_string()));
    assert_eq!(fh.pop(), Some("a5".to_string()));
    assert_eq!(fh.pop(), Some("a6".to_string()));
    assert_eq!(fh.pop(), Some("a7".to_string()));
    assert_eq!(fh.pop(), None);
}

#[test]
#[should_panic]
fn test_decrease_key_3() {
    let mut fh: FibonacciHeap<String> = FibonacciHeap::new();
    let mut fh_wrong: FibonacciHeap<String> = FibonacciHeap::new();
    fh.push("a1".to_string());
    fh.push("a2".to_string());
    fh.push("a3".to_string());
    let a4_ptr = fh.push("a4".to_string());
    fh.push("a5".to_string());
    fh.push("a6".to_string());
    fh.push("a7".to_string());
    assert_eq!(fh.pop(), Some("a1".to_string()));
    assert_eq!(fh.pop(), Some("a2".to_string()));
    fh_wrong.decrease_key(&a4_ptr, "a0".to_string()); // wrong heap: panic
}

#[test]
fn test_decrease_key_4() {
    let mut fh: FibonacciHeap<String> = FibonacciHeap::new();
    fh.push("a1".to_string());
    fh.push("a2".to_string());
    fh.push("a3".to_string());
    let a4_ptr = fh.push("a4".to_string());
    fh.push("a5".to_string());
    fh.push("a6".to_string());
    fh.push("a7".to_string());
    assert_eq!(fh.pop(), Some("a1".to_string()));
    assert_eq!(fh.pop(), Some("a2".to_string()));
    // can do it several times
    fh.decrease_key(&a4_ptr, "a3".to_string());
    fh.decrease_key(&a4_ptr, "a2".to_string());
    fh.decrease_key(&a4_ptr, "a1".to_string());
    fh.decrease_key(&a4_ptr, "a0".to_string());
    assert_eq!(fh.pop(), Some("a0".to_string()));
    assert_eq!(fh.pop(), Some("a3".to_string()));
    assert_eq!(fh.pop(), Some("a5".to_string()));
    assert_eq!(fh.pop(), Some("a6".to_string()));
    assert_eq!(fh.pop(), Some("a7".to_string()));
    assert_eq!(fh.pop(), None);
}

#[test]
fn test_decrease_key_5() {
    let mut fh: FibonacciHeap<String> = FibonacciHeap::new();
    fh.push("a1".to_string());
    fh.push("a2".to_string());
    fh.push("a3".to_string());
    let a4_ptr = fh.push("a4".to_string());
    fh.push("a5".to_string());
    fh.push("a6".to_string());
    fh.push("a7".to_string());
    assert_eq!(fh.pop(), Some("a1".to_string()));
    assert_eq!(fh.pop(), Some("a2".to_string()));
    assert_eq!(fh.pop(), Some("a3".to_string()));
    assert_eq!(fh.pop(), Some("a4".to_string()));
    fh.decrease_key(&a4_ptr, "a3".to_string()); // already popped: nothing happens
    assert_eq!(fh.pop(), Some("a5".to_string()));
    assert_eq!(fh.pop(), Some("a6".to_string()));
    assert_eq!(fh.pop(), Some("a7".to_string()));
    assert_eq!(fh.pop(), None);
}

#[test]
fn test_delete_1() {
    let mut fh: FibonacciHeap<String> = FibonacciHeap::new();
    fh.push("a1".to_string());
    fh.push("a2".to_string());
    fh.push("a3".to_string());
    let a4_ptr = fh.push("a4".to_string());
    fh.push("a5".to_string());
    fh.push("a6".to_string());
    fh.push("a7".to_string());
    assert_eq!(fh.pop(), Some("a1".to_string()));
    assert_eq!(fh.pop(), Some("a2".to_string()));
    fh.delete(a4_ptr);
    assert_eq!(fh.pop(), Some("a3".to_string()));
    assert_eq!(fh.pop(), Some("a5".to_string()));
    assert_eq!(fh.pop(), Some("a6".to_string()));
    assert_eq!(fh.pop(), Some("a7".to_string()));
    assert_eq!(fh.pop(), None);
}

#[test]
fn test_delete_2() {
    let mut fh: FibonacciHeap<String> = FibonacciHeap::new();
    let a1_ptr = fh.push("a1".to_string());
    let a2_ptr = fh.push("a1".to_string());
    let a3_ptr = fh.push("a1".to_string());
    let a4_ptr = fh.push("a1".to_string());
    let a5_ptr = fh.push("a1".to_string());
    let a6_ptr = fh.push("a1".to_string());
    let a7_ptr = fh.push("a1".to_string());
    fh.delete(a1_ptr);
    fh.delete(a2_ptr);
    fh.delete(a3_ptr);
    fh.delete(a4_ptr);
    fh.delete(a5_ptr);
    fh.delete(a6_ptr);
    fh.delete(a7_ptr);
    assert_eq!(fh.pop(), None);
}

#[test]
fn test_delete_3() {
    let mut fh: FibonacciHeap<String> = FibonacciHeap::new();
    let a1_ptr = fh.push("a1".to_string());
    let a2_ptr = fh.push("a1".to_string());
    let _ = fh.push("a3".to_string());
    let a4_ptr = fh.push("a1".to_string());
    let a5_ptr = fh.push("a1".to_string());
    let a6_ptr = fh.push("a1".to_string());
    let a7_ptr = fh.push("a1".to_string());
    fh.delete(a1_ptr);
    fh.delete(a2_ptr);
    fh.delete(a4_ptr);
    fh.delete(a5_ptr);
    fh.delete(a6_ptr);
    fh.delete(a7_ptr);
    assert_eq!(fh.pop(), Some("a3".to_string()));
    assert_eq!(fh.pop(), None);
}

#[test]
#[should_panic]
fn test_delete_4() {
    let mut fh: FibonacciHeap<String> = FibonacciHeap::new();
    let mut fh_wrong: FibonacciHeap<String> = FibonacciHeap::new();
    fh.push("a1".to_string());
    fh.push("a2".to_string());
    fh.push("a3".to_string());
    let a4_ptr = fh.push("a4".to_string());
    fh.push("a5".to_string());
    fh.push("a6".to_string());
    fh.push("a7".to_string());
    assert_eq!(fh.pop(), Some("a1".to_string()));
    assert_eq!(fh.pop(), Some("a2".to_string()));
    fh_wrong.delete(a4_ptr); // wrong heap: panic
}

#[test]
fn test_delete_5() {
    let mut fh: FibonacciHeap<String> = FibonacciHeap::new();
    fh.push("a1".to_string());
    fh.push("a2".to_string());
    fh.push("a3".to_string());
    let a4_ptr = fh.push("a4".to_string());
    fh.push("a5".to_string());
    fh.push("a6".to_string());
    fh.push("a7".to_string());
    assert_eq!(fh.pop(), Some("a1".to_string()));
    assert_eq!(fh.pop(), Some("a2".to_string()));
    fh.delete(a4_ptr.clone()); // delete multiple times is OK
    fh.delete(a4_ptr.clone()); // (nothing happens after first time)
    fh.delete(a4_ptr.clone());
    fh.delete(a4_ptr.clone());
    fh.delete(a4_ptr.clone());
    fh.delete(a4_ptr.clone());
    fh.delete(a4_ptr.clone());
    fh.delete(a4_ptr.clone());
    fh.delete(a4_ptr.clone());
    fh.delete(a4_ptr.clone());
    fh.delete(a4_ptr.clone());
    fh.delete(a4_ptr.clone());
    fh.delete(a4_ptr.clone());
    fh.delete(a4_ptr.clone());
    fh.delete(a4_ptr.clone());
    fh.delete(a4_ptr.clone());
    fh.delete(a4_ptr.clone());
    assert_eq!(fh.pop(), Some("a3".to_string()));
    assert_eq!(fh.pop(), Some("a5".to_string()));
    assert_eq!(fh.pop(), Some("a6".to_string()));
    assert_eq!(fh.pop(), Some("a7".to_string()));
    assert_eq!(fh.pop(), None);
}

#[test]
fn test_delete_6() {
    let mut fh: FibonacciHeap<String> = FibonacciHeap::new();
    let ptr = fh.push("a1".to_string());
    fh.delete(ptr.clone());
    assert_eq!(fh.pop(), None);
}

#[test]
fn test_delete_7() {
    let mut fh: FibonacciHeap<String> = FibonacciHeap::new();
    let ptr = fh.push("a1".to_string());
    fh.push("a2".to_string());
    fh.delete(ptr);
    assert_eq!(fh.pop(), Some("a2".to_string()));
    assert_eq!(fh.pop(), None);
}

#[test]
fn test_delete_8() {
    let mut fh: FibonacciHeap<String> = FibonacciHeap::new();
    fh.push("a1".to_string());
    let ptr = fh.push("a2".to_string());
    fh.delete(ptr);
    assert_eq!(fh.pop(), Some("a1".to_string()));
    assert_eq!(fh.pop(), None);
}

#[test]
fn test_delete_9() {
    let mut fh: FibonacciHeap<String> = FibonacciHeap::new();
    let ptr1 = fh.push("a1".to_string());
    let ptr2 = fh.push("a2".to_string());
    fh.delete(ptr1);
    fh.delete(ptr2);
    assert_eq!(fh.pop(), None);
}

#[test]
fn test_delete_10() {
    let mut fh: FibonacciHeap<String> = FibonacciHeap::new();
    let ptr1 = fh.push("a1".to_string());
    let ptr2 = fh.push("a2".to_string());
    fh.delete(ptr2);
    fh.delete(ptr1);
    assert_eq!(fh.pop(), None);
}
