use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct MedianFinder {
    pub min_heap: BinaryHeap<Reverse<i32>>,
    pub max_heap: BinaryHeap<i32>,
}

impl MedianFinder {
    fn new() -> Self {
        MedianFinder {
            min_heap: BinaryHeap::new(),
            max_heap: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        if self.max_heap.len() == self.min_heap.len() {
            self.max_heap.push(num);
            if let Some(e) = self.max_heap.pop() {
                self.min_heap.push(Reverse(e))
            }
        } else {
            self.min_heap.push(Reverse(num));
            if let Some(e) = self.min_heap.pop() {
                self.max_heap.push(e.0)
            }
        }
    }

    fn find_median(&self) -> f64 {
        match (self.min_heap.peek(), self.max_heap.peek()) {
            (Some(Reverse(m)), Some(n)) => if self.min_heap.len() > self.max_heap.len() {
                *m as f64
            } else {
                (m + n) as f64 / 2 as f64
            },
            (Some(Reverse(m)), None) => *m as f64,
            (None, Some(n)) => *n as f64,
            (None, None) => 0.0,
        }
    }
}
