pub struct BinaryMinHeap{
    heap_size : usize,
    arr : Vec<i32> 
}

impl BinaryMinHeap {
    pub fn new(size : usize) -> BinaryMinHeap {
        //let temp = [0;size];
        //BinaryMinHeap{heap_size: 0, arr: temp}
        BinaryMinHeap{heap_size: 0, arr: vec![0;size]} 
    }
        
    pub fn insert(&mut self, new_data :i32) {
        if self.arr.len() == self.heap_size {
            panic!("Heap Overflow!");
        } else {
            self.heap_size += 1;
            let heap_index = self.heap_size -1;
            self.arr[heap_index] = new_data;
            self.shift_up(heap_index);
        }
    }

    fn shift_up(&mut self, node_index : usize) {
        if node_index !=0 {
            let parent_index = self.get_parent_index(node_index);
            if self.arr[parent_index] > self.arr[node_index] {
                self.swap(parent_index, node_index);
                self.shift_up(parent_index);
            }
        }
    }

    fn get_parent_index(&self, node_index : usize) -> usize {
        (node_index - 1) / 2
    }

    fn swap(&mut self, i : usize, j :usize) {
        let temp = self.arr[i];
        self.arr[i] = self.arr[j];
        self.arr[j] = temp;
    }

    pub fn print(&self) {
       for (index, value) in self.arr.iter().enumerate() {
           println!("index : {}, value : {}", index, value);
       }
    }
}
