use std::collections::BinaryHeap;
use std::cmp::Reverse;

/*
The median is the middle value in an ordered integer list. If the size of the list is even, there is no middle value and the median is the mean of the two middle values.

For example, for arr = [2,3,4], the median is 3.
For example, for arr = [2,3], the median is (2 + 3) / 2 = 2.5.
Implement the MedianFinder class:

MedianFinder() initializes the MedianFinder object.
void addNum(int num) adds the integer num from the data stream to the data structure.
double findMedian() returns the median of all elements so far. Answers within 10-5 of the actual answer will be accepted.
*/

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
