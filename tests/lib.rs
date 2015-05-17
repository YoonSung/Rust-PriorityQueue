extern crate rust_datastructure;
use rust_datastructure::binary_min_heap;

#[test]
fn binary_min_heap() {
    let mut unordered_array = [9,8,7,6,5,4,3,2,1,0];    

    binary_min_heap::test();
}
