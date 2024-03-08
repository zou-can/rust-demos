#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> ListNode {
        ListNode {
            val,
            next: None,
        }
    }
}

/// 反转链表
pub fn reverse(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // 上一个 node
    let mut prev = None;
    // 当前 node
    let mut curr = head;

    while let Some(mut boxed_node) = curr.take() {
        // 获取下一个 node
        let next = boxed_node.next.take();

        // 反转当前 node：指向前一个 node
        boxed_node.next = prev.take();

        // 向前移动 prev，current
        prev = Some(boxed_node);
        curr = next;
    }

    // 当前 node 为 None 时结束，所以返回前一个 node
    prev
}

/// 检测是否为环形链表
/// 快慢指针
pub fn is_cycle(head: &Option<Box<ListNode>>) -> bool {
    let mut fast_p = head;
    let mut slow_p = head;
    while fast_p.is_some() && fast_p.as_ref().unwrap().next.is_some() {
        slow_p = &slow_p.as_ref().unwrap().next;
        fast_p = &fast_p.as_ref().unwrap().next.as_ref().unwrap().next;

        if slow_p == fast_p {
            return true;
        }
    }

    false
}

/// 合并两个有序链表
pub fn merge_sorted_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (Some(mut node1), Some(mut node2)) => {
            if node1.val < node2.val {
                let n = node1.next.take();
                node1.next = merge_sorted_lists(n, Some(node2));
                Some(node1)
            } else {
                let n = node2.next.take();
                node2.next = merge_sorted_lists(Some(node1), n);
                Some(node2)
            }
        }
        (x, y) => x.or(y),
    }
}

// utils

fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current = None;
    for v in vec.into_iter().rev() {
        let mut node = ListNode::new(v);
        node.next = current;
        current = Some(Box::new(node));
    }

    current
}

fn to_vec(node: Option<Box<ListNode>>) -> Vec<i32> {
    let mut vec = Vec::new();
    let mut current = node;

    while let Some(mut boxed_node) = current.take() {
        vec.push(boxed_node.val);
        current = boxed_node.next.take();
    }

    vec
}

#[cfg(test)]
mod tests {
    use crate::linked_list::{is_cycle, ListNode, merge_sorted_lists, reverse, to_list, to_vec};

    #[test]
    fn test_reverse() {
        let input = vec![1, 2, 3, 4, 5];

        let mut expected = input.clone();
        expected.reverse();

        let result = reverse(to_list(vec![1, 2, 3, 4, 5]));

        assert_eq!(expected, to_vec(result));
    }

    #[test]
    fn test_is_cycle() {
        let no_cycle = to_list(vec![1, 2, 3, 4, 5]);

        assert!(!is_cycle(&no_cycle));

        // 无法测试循环链表，因为在 Box<ListNode> 定义下无法持有多个值，需要使用 Rc 修改 ListNode 的定义
    }

    #[test]
    fn test_merge_sorted_list() {
        let result = merge_sorted_lists(to_list(vec![1, 3, 4]), to_list(vec![1, 2, 4]));

        assert_eq!(vec![1, 1, 2, 3, 4, 4], to_vec(result));
    }
}