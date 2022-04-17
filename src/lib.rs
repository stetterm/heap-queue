///
/// Module containing the implementation
/// of a sorted queue. Uses a min/max 
/// heap to facilitate efficient queue
/// push and pop operations.
/// 
pub mod sorted_queue {
    use std::collections::HashMap;


    ///
    /// A sorted priority queue that uses
    /// either a max heap or a min heap.
    /// Uses a vector to store references to
    /// a generic data type. The map will return
    /// the index of the value in the heap. The heap
    /// stores the value it is compared by and
    /// the index of the reference associated
    /// with the value.
    /// 
    pub struct SortedQueue<'a, T, F>
    where T: PartialOrd + Ord + Copy,
          F: Eq + std::hash::Hash
    {
        heap: Vec<(T, usize)>,
        comp: Box<dyn Fn((T, usize), (T, usize)) -> bool>,
        data: Vec<&'a F>,
        map: HashMap<&'a F, usize>,
    }

    impl<'a, T, F> SortedQueue<'a, T, F>
    where T: PartialOrd + Ord + Copy,
          F: Eq + std::hash::Hash
    {
        ///
        /// Returns a new, blank priority queue.
        /// 
        pub fn new(max: bool) -> SortedQueue<'a, T, F> {
            if max {
                SortedQueue {
                    heap: vec![],
                    comp: Box::new(|x, y| {
                        let (a, _) = x;
                        let (b, _) = y;
                        a > b
                    }),
                    data: vec![],
                    map: HashMap::new(),
                }
            } else {
                SortedQueue {
                    heap: vec![],
                    comp: Box::new(|x, y| {
                        let (a, _) = x;
                        let (b, _) = y;
                        a < b
                    }),
                    data: vec![],
                    map: HashMap::new(),
                }
            }
        }

        fn swap(&mut self, i1: usize, i2: usize) {
            let (_, o1) = self.heap[i1];
            let (_, o2) = self.heap[i2];
            let o1 = self.data[o1];
            let o2 = self.data[o2];
            self.map.insert(o1, i2);
            self.map.insert(o2, i1);
            let temp = self.heap[i1];
            self.heap[i1] = self.heap[i2];
            self.heap[i2] = temp;
        }

        fn sift_down(&mut self, index: usize) {
            if self.heap.len() <= 1 {
                return;
            }
            let mut index = index;
            let mut swap_index = index;
            loop {
                swap_index = 2 * index + 1;
                if swap_index >= self.heap.len() {
                    break;
                }
                if swap_index + 1 == self.heap.len() 
                    || (self.comp)(self.heap[swap_index], self.heap[swap_index + 1]) {
                    if (self.comp)(self.heap[swap_index], self.heap[index]) {
                        self.swap(index, swap_index);
                        index = swap_index;
                    } else {
                        break;
                    }
                } else if (self.comp)(self.heap[swap_index + 1], self.heap[index]) {
                    self.swap(index, swap_index + 1);
                    index = swap_index + 1;
                } else {
                    break;
                }
            }
        }

        fn sift_up(&mut self, index: usize) {
            if self.heap.len() <= 1 || index == 0 {
                return;
            }
            let mut index = index;
            let mut swap_index = index;
            loop {
                swap_index = (index - 1) / 2;
                if (self.comp)(self.heap[index], self.heap[swap_index]) {
                    self.swap(index, swap_index);
                } else {
                    break;
                }
                if swap_index == 0 {
                    break;
                }
            }
        }

        pub fn enq(&mut self, value: T, data: &'a F) {
            let index = self.data.len();
            self.data.push(&data);
            let new_index = self.heap.len();
            self.heap.push((value, index));
            self.map.insert(&data, new_index);
            self.sift_up(new_index);
        }

        pub fn deq(&mut self) -> Option<(T, &'a F)> {
            if self.heap.len() == 0 {
                return None;
            }

            self.swap(0, self.heap.len()-1);
            self.sift_down(0);
            let (rem_val, rem_index) = self.heap.remove(self.heap.len()-1);

            let swap_val = self.map.get(self.data[self.data.len()-1])?;
            let (swap_heap_val, _) = self.heap[*swap_val];
            self.heap[*swap_val] = (swap_heap_val, rem_index);

            let temp = self.data[rem_index];
            let length = self.data.len();
            self.data[rem_index] = self.data[length-1];
            self.data[length-1] = temp;
            
            let ret_val = self.data.remove(self.data.len()-1);
            Some((rem_val, ret_val))
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn compare_tuple() {
        let x = (4, "hello");
        let y = (4, "there");
        println!("{}", x > y);
    }
}