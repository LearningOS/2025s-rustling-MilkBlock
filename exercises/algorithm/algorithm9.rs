/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Heap<T>
where
    T: Default + Debug 
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default + Debug,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn add(&mut self, value: T) {
        self.count+=1;
        self.items.push(value);
        let mut child = self.count;
        let mut parent = self.parent_idx(child);
        while (self.comparator)(&self.items[child],&self.items[parent]) && parent!=0{
            self.items.swap(child, parent);
            child = parent;
            parent = self.parent_idx(child);
        }
        println!("{:?}",self);
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

}

impl<T> Heap<T>
where
    T: Default + Ord + Debug,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T> 
where
    T: Default + Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
        if self.count ==0 {
            return None;
        }
        if self.count ==1 {
            return self.items.pop();
        }
        let poped = self.items.pop().unwrap();
        let taken = std::mem::replace(self.items.get_mut(1).unwrap(), poped);

        let mut parent = 1;   
        let mut l_child = self.left_child_idx(parent);   
        let mut r_child = self.right_child_idx(parent);   

        loop{
            let p = self.items.get(parent).unwrap();
            match (self.items.get(l_child),self.items.get(r_child)){
                (None, None) => {break;},
                (None, Some(r)) => {
                    if (self.comparator)(r,p){
                        self.items.swap(r_child,parent);
                        parent = r_child;   
                    }else {
                        break;
                    }
                },
                (Some(l), None) => {
                    if (self.comparator)(l,p){
                        self.items.swap(l_child,parent);
                        parent = l_child;   
                    }else {
                        break;
                    }
                },
                (Some(l), Some(r)) => {
                    // 
                    if (self.comparator)(l,r) && (self.comparator)(l,p){
                        self.items.swap(l_child,parent);
                        parent = l_child;   
                    }else if (self.comparator)(r,p){
                        self.items.swap(r_child,parent);
                        parent = r_child;   
                    }else {
                        break;
                    }
                },
            }
            l_child = self.left_child_idx(parent);   
            r_child = self.right_child_idx(parent);   
        }
        self.count -= 1;
        println!("{:?}",self);
        Some(taken)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Debug,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Debug,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}