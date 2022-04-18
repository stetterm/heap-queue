///
/// Module containing the implementation
/// of a sorted queue. Uses a min/max 
/// heap to facilitate efficient queue
/// push and pop operations.
/// 
pub mod sorted_queue {
    use std::collections::HashMap;

    ///
    /// Custom error type returned if
    /// the value to search for does
    /// not appear in the queue.
    /// 
    #[derive(Debug)]
    pub struct NotInQueue;

    impl std::fmt::Display for NotInQueue {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Value not found in queue")
        }
    }

    impl std::error::Error for NotInQueue {}

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
        heap: Vec<(T, &'a F)>,
        comp: Box<dyn Fn((T, &'a F), (T, &'a F)) -> bool>,
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
                    map: HashMap::new(),
                }
            }
        }

        fn swap(&mut self, i1: usize, i2: usize) {
            let (_, o1) = self.heap[i1];
            let (_, o2) = self.heap[i2];
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

        ///
        /// Puts the value and the associated
        /// referrence in the pritority
        /// queue.
        /// 
        pub fn enq(&mut self, value: T, data: &'a F) {
            let new_index = self.heap.len();
            self.heap.push((value, data));
            self.map.insert(data, new_index);
            self.sift_up(new_index);
        }

        ///
        /// Removes the value from the front
        /// of the queue and returns the value
        /// and associated referrence.
        /// 
        pub fn deq(&mut self) -> Option<(T, &'a F)> {
            if self.heap.len() == 0 {
                return None;
            }
            self.swap(0, self.heap.len()-1);
            let (rem_val, rem_ref) = self.heap.remove(self.heap.len()-1);
            self.sift_down(0);
            Some((rem_val, rem_ref))
        }

        ///
        /// Tries to find the data object in the
        /// queue and change its priority value,
        /// fails if this value cannot be
        /// found in the queue.
        /// 
        pub fn change_priority(&mut self, new_value: T, data: &'a F) 
            -> Result<(), Box<dyn std::error::Error>> {
            let index = match self.map.get(data) {
                Some(i) => *i,
                None => return Err(Box::new(NotInQueue)),
            };
            let (old_val, _) = self.heap[index];
            self.heap[index] = (new_value, data);
            if (self.comp)((old_val, data), (new_value, data)) {
                self.sift_down(index);
            } else {
                self.sift_up(index);
            }
            Ok(())
        }

        ///
        /// Tries to find the value in the
        /// queue with the reference, and sets
        /// it to the new reference.
        /// 
        pub fn set_ref(&mut self, data: &'a F, new_ref: &'a F)
            -> Result<(), Box<dyn std::error::Error>> {
            let index = match self.map.get(data) {
                Some(i) => *i,
                None => return Err(Box::new(NotInQueue)),
            };
            let (old_val, _) = self.heap[index];
            self.heap[index] = (old_val, new_ref);
            self.map.remove(data);
            self.map.insert(new_ref, index);
            Ok(())
        }

        ///
        /// Returns the number of elements
        /// in the priority queue.
        /// 
        pub fn size(&self) -> usize {
            self.heap.len()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::sorted_queue::*;

    #[test]
    fn compare_tuple() {
        let x = (4, "hello");
        let y = (4, "there");
        println!("{}", x > y);
    }

    #[derive(Hash, Eq, Debug)]
    struct Employee<'a> {
        name: &'a str,
        id: u64,
    }

    impl<'a> PartialEq for Employee<'a> {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }

    #[test]
    fn simple_queue() {
        let emp_list = vec![
            Employee { name: "victor", id: 10237 },
            Employee { name: "hugo", id: 2937 },
            Employee { name: "marcus", id: 1902 },
        ];
        let mut queue = SortedQueue::new(true);
        for i in 0..3 {
            queue.enq(i, &emp_list[i]);
        }
        for i in 0..3 {
            let (_, emp) = queue.deq().unwrap();
            dbg!(emp);
        }
    }
}