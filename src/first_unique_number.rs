/*
First Unique Number

You have a queue of integers, you need to retrieve the first unique integer in the queue.

Implement the FirstUnique class:
    1. FirstUnique(int[] nums) Initializes the object with the numbers in the queue.
    2. int showFirstUnique() returns the value of the first unique integer of the queue, and returns -1 if there is no such integer.
    3. void add(int value) insert value to the queue.

Example 1:
Input:
["FirstUnique","showFirstUnique","add","showFirstUnique","add","showFirstUnique","add","showFirstUnique"]
[[[2,3,5]],[],[5],[],[2],[],[3],[]]
Output:
[null,2,null,2,null,3,null,-1]
Explanation:
FirstUnique firstUnique = new FirstUnique([2,3,5]);
firstUnique.showFirstUnique(); // return 2
firstUnique.add(5);            // the queue is now [2,3,5,5]
firstUnique.showFirstUnique(); // return 2
firstUnique.add(2);            // the queue is now [2,3,5,5,2]
firstUnique.showFirstUnique(); // return 3
firstUnique.add(3);            // the queue is now [2,3,5,5,2,3]
firstUnique.showFirstUnique(); // return -1

Example 2:
Input:
["FirstUnique","showFirstUnique","add","add","add","add","add","showFirstUnique"]
[[[7,7,7,7,7,7]],[],[7],[3],[3],[7],[17],[]]
Output:
[null,-1,null,null,null,null,null,17]
Explanation:
FirstUnique firstUnique = new FirstUnique([7,7,7,7,7,7]);
firstUnique.showFirstUnique(); // return -1
firstUnique.add(7);            // the queue is now [7,7,7,7,7,7,7]
firstUnique.add(3);            // the queue is now [7,7,7,7,7,7,7,3]
firstUnique.add(3);            // the queue is now [7,7,7,7,7,7,7,3,3]
firstUnique.add(7);            // the queue is now [7,7,7,7,7,7,7,3,3,7]
firstUnique.add(17);           // the queue is now [7,7,7,7,7,7,7,3,3,7,17]
firstUnique.showFirstUnique(); // return 17

Example 3:
Input:
["FirstUnique","showFirstUnique","add","showFirstUnique"]
[[[809]],[],[809],[]]
Output:
[null,809,null,-1]
Explanation:
FirstUnique firstUnique = new FirstUnique([809]);
firstUnique.showFirstUnique(); // return 809
firstUnique.add(809);          // the queue is now [809,809]
firstUnique.showFirstUnique(); // return -1

Constraints:
1 <= nums.length <= 10^5
1 <= nums[i] <= 10^8
1 <= value <= 10^8
At most 50000 calls will be made to showFirstUnique and add.
*/

use std::collections::HashMap;

pub struct FirstUnique {
    queue: Vec<i32>,
    map: HashMap<i32, i32>,
}

impl FirstUnique {
    fn new(nums: Vec<i32>) -> Self {
        let mut queue = vec![];
        let mut map = HashMap::new();
        for i in nums {
            *map.entry(i).or_insert(0) += 1;
            if map.get(&i).unwrap() == &1 {
                queue.push(i);
            }
        }

        FirstUnique {
            queue,
            map,
        }
    }

    fn show_first_unique(&mut self) -> i32 {
        while !self.queue.is_empty() {
            let first = self.queue.first().unwrap();

            if self.map.get(first).unwrap() > &1 {
                self.queue.remove(0);
            } else {
                return first.clone() as i32;
            }
        }
        -1
    }

    fn add(&mut self, value: i32) {
        *self.map.entry(value).or_insert(0) += 1;
        if self.map.get(&value).unwrap() == &1 {
            self.queue.push(value);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let mut first_unique = FirstUnique::new(vec![2, 3, 5]);
        assert_eq!(first_unique.show_first_unique(), 2);
        first_unique.add(5);
        assert_eq!(first_unique.show_first_unique(), 2);
        first_unique.add(2);
        assert_eq!(first_unique.show_first_unique(), 3);
        first_unique.add(3);
        assert_eq!(first_unique.show_first_unique(), -1);
    }

    #[test]
    fn test_2() {
        let mut first_unique = FirstUnique::new(vec![7, 7, 7, 7, 7, 7]);
        assert_eq!(first_unique.show_first_unique(), -1);
        first_unique.add(7);
        first_unique.add(7);
        first_unique.add(3);
        first_unique.add(3);
        first_unique.add(17);
        assert_eq!(first_unique.show_first_unique(), 17);
    }

    #[test]
    fn test_3() {
        let mut first_unique = FirstUnique::new(vec![809]);
        assert_eq!(first_unique.show_first_unique(), 809);
        first_unique.add(809);
        assert_eq!(first_unique.show_first_unique(), -1);
    }
}