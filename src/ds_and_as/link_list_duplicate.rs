use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn get_tree_from_slice(slice: Vec<i32>) -> Option<Box<ListNode>> {
    if slice.is_empty() {
        return None;
    }
    let mut list = Box::new(ListNode::new(*slice.first().unwrap()));
    let arr_slice = &slice[1..];

    fn set_next(head: &mut ListNode, arr: Vec<i32>) {
        if arr.is_empty() {
            return;
        }
        let val = *arr.first().unwrap();
        let arr_slice = &arr[1..];

        let next = ListNode::new(val);
        head.next = Some(Box::new(next));
        set_next(&mut *head.next.as_mut().unwrap(), Vec::from(arr_slice));
    }
    set_next(&mut list, Vec::from(arr_slice));
    Some(list)
}

pub struct Solution;

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut history: HashMap<i32, bool> = HashMap::new();
        let mut head = head;
        Solution::remove(&mut head, &mut history);
        head
    }

    fn remove(head: &mut Option<Box<ListNode>>, history: &mut HashMap<i32, bool>) {
        if head.is_none() {
            return;
        }

        let head_val = head.as_ref().unwrap().val;

        match history.get(&head_val) {
            Some(_) => {
                let next = &head.as_mut().unwrap().next;
                match next {
                    Some(list) => {
                        if list.val == head_val {
                            let new_list = list.next.clone();
                            head.as_mut().unwrap().next = new_list;
                            Solution::remove(head, history);
                        }
                    }
                    None => (),
                }
            }
            None => {
                history.insert(head_val, true);
                let next_head = &head.as_mut().unwrap().next;
                match next_head {
                    Some(list) => {
                        if list.val == head_val {
                            let new_list = list.next.clone();
                            head.as_mut().unwrap().next = new_list;
                            Solution::remove(head, history);
                        }
                    }
                    None => (),
                }
            }
        }
        let next_head = &mut head.as_mut().unwrap().next;
        Solution::remove(next_head, history);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let head = get_tree_from_slice(Vec::from([1, 1, 2]));
        let result = Solution::delete_duplicates(head);
        let expected = get_tree_from_slice(Vec::from([1, 2]));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_2() {
        let head = get_tree_from_slice(Vec::from([1, 1, 2, 3, 3]));
        let result = Solution::delete_duplicates(head);
        let expected = get_tree_from_slice(Vec::from([1, 2, 3]));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_3() {
        let head = get_tree_from_slice(Vec::from([1, 1, 1]));
        let result = Solution::delete_duplicates(head);
        let expected = get_tree_from_slice(Vec::from([1]));
        assert_eq!(result, expected);
    }
}
