///
/// Module containing the implementation
/// of a sorted queue. Uses a min/max 
/// heap to facilitate efficient queue
/// push and pop operations.
/// 
pub mod sorted_queue {

    pub struct SortedQueue<T>
    where T: PartialOrd + Ord + Copy
    {
        heap: Vec<T>,
        cmp: Box<dyn Fn(T, T) -> bool>,
    }

    impl<T> SortedQueue<T>
    where T: PartialOrd + Ord + Copy
    {
        pub fn new(max: bool) -> SortedQueue<T> {
            if max {
                SortedQueue {
                    heap: vec![],
                    cmp: Box::new(|x, y| x > y),
                }
            } else {
                SortedQueue {
                    heap: vec![],
                    cmp: Box::new(|x, y| x < y),
                }
            }
        }

        fn swap(&mut self, i1: usize, i2: usize) {
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
                    || self.heap[swap_index] > self.heap[swap_index + 1] {
                    if self.heap[swap_index] > self.heap[index] {
                        self.swap(index, swap_index);
                        index = swap_index;
                    } else {
                        break;
                    }
                } else if self.heap[swap_index + 1] > self.heap[index] {
                    self.swap(index, swap_index + 1);
                    index = swap_index + 1;
                } else {
                    break;
                }
            }
        }

        
    }
}