extern crate add_macro;
use add_macro::{ deq, map, set, heap, list, btree_map, btree_set };
use std::collections::*;

#[test]
fn test_vec_deque() {
    assert_eq!(
        deq![1, 2, 3, 4],
        VecDeque::from([1, 2, 3, 4])
    );

    assert_eq!(
        deq![0u8; 10].len(),
        10
    );
}

#[test]
fn test_hash_map() {
    assert_eq!(
        map! {
            "key1" => "value1",
            "key2" => "value2",
        },
        HashMap::from([("key1", "value1"), ("key2", "value2")])
    );
}

#[test]
fn test_hash_set() {
    assert_eq!(
        set![1, 2, 3, 4],
        {
            let mut set = HashSet::new();
            set.insert(1);
            set.insert(2);
            set.insert(3);
            set.insert(4);
            set
        }
    );
}

#[test]
fn test_binary_heap() {
    assert_eq!(
        heap![&1, &2, &3, &4].pop(),
        BinaryHeap::from([&1, &2, &3, &4]).pop()
    );

    assert_eq!(
        heap![&1; 10].len(),
        10
    );
}

#[test]
fn test_linked_list() {
    assert_eq!(
        list![1, 2, 3, 4],
        LinkedList::from([1, 2, 3, 4])
    );
    
    assert_eq!(
        list![0u8; 10].len(),
        10
    );
}

#[test]
fn test_btree_map() {
    assert_eq!(
        btree_map! {
            "key1" => "value1",
            "key2" => "value2",
        },
        BTreeMap::from([("key1", "value1"), ("key2", "value2")])
    );
}

#[test]
fn test_btree_set() {
    assert_eq!(
        btree_set![1, 2, 3, 4],
        {
            let mut set = BTreeSet::new();
            set.insert(1);
            set.insert(2);
            set.insert(3);
            set.insert(4);
            set
        }
    );
}
