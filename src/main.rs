mod declaration;

use std::{
    collections::{self, HashMap, HashSet},
    i32::MAX,
    string,
    usize::MIN,
    vec,
};

use declaration::*;

fn main() {
    print!("hi");
}

/**
 *  01
 */
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = std::collections::HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        if let Some(&j) = map.get(&(target - num)) {
            return vec![j as i32, i as i32];
        } else {
            map.insert(num, i);
        }
    }
    vec![]
}
/**
 * 02
 */
pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut p = l1.as_deref();
    let mut q = l2.as_deref();
    let mut carry: bool = false;
    let mut d_head = Box::new(ListNode { val: 0, next: None });
    let mut cur = &mut d_head;
    while p.is_some() || q.is_some() || carry {
        let x = p.map_or(0, |node| node.val);
        let y = q.map_or(0, |node| node.val);
        let sum = x + y + if carry { 1 } else { 0 };
        carry = sum >= 10;
        cur.next = Some(Box::new(ListNode {
            val: sum % 10,
            next: None,
        }));
        cur = cur.next.as_mut().unwrap();
        p = p.and_then(|node| node.next.as_deref());
        q = q.and_then(|node| node.next.as_deref());
    }
    d_head.next
}

/**
 * 03
 */
pub fn length_of_longest_substring(s: String) -> i32 {
    match s.len() {
        0 => 0,
        1 => 1,
        _ => {
            let mut hm: HashMap<&char, usize> = std::collections::HashMap::new();
            let char_vec: Vec<char> = s.chars().collect();
            let mut length = 0;
            let mut last_idx = 0;
            for (idx, char) in char_vec.iter().enumerate() {
                if let Some(&prev_idx) = hm.get(&char) {
                    length = std::cmp::max(length, idx - last_idx);
                    last_idx = std::cmp::max(prev_idx + 1, last_idx);
                }
                hm.insert(char, idx);
            }
            length = std::cmp::max(length, char_vec.len() - last_idx);
            length as i32
        }
    }
}
/**
 * 04
 */
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let size = nums1.len() + nums2.len();
    let mid = size / 2 - 1;
    let (mut i, mut j) = (0usize, 0usize);
    let (mut lo, mut hi) = (0usize, nums1.len());
    while lo < hi {
        i = (lo + hi) / 2;
        j = mid - i;
        if i > nums1.len() - 1
            || nums1.get(i).unwrap_or_else(|| &i32::MAX) < nums2.get(j).unwrap_or_else(|| &i32::MIN)
        {
            // shift i left
            hi = i - 1;
            continue;
        }
        if j > nums2.len() - 1 || nums2[j] < nums1[i] {
            // shift i right
            lo = i + 1;
            continue;
        }
        break;
    }
    if size % 2 == 0 {
        (nums1[i] + nums2[j]) as f64 / 2.0
    } else {
        nums1[i] as f64
    }
}

pub fn is_palindrome(s: String) -> bool {
    let iter = s.chars().into_iter();
    let filtered: Vec<char> = iter
        .filter(|c| c.is_alphabetic() || c.is_numeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    if filtered.len() < 2 {
        return true;
    }
    let mut l = 0usize;
    let mut r = filtered.len() - 1;
    while l < r {
        if filtered[l] != filtered[r] {
            return false;
        }
        l += 1;
        r -= 1;
    }
    return true;
}
pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
    let s: HashSet<String> = HashSet::from_iter(banned);
    let ex: HashSet<char> =
        HashSet::from_iter(vec!['"', '!', '?', '\'', ',', ';', '.', '"'].into_iter());
    let words = paragraph
        .chars()
        .into_iter()
        .map(|c| if ex.contains(&c) { ' ' } else { c })
        .map(|c| c.to_ascii_lowercase())
        .fold(Vec::new(), |mut acc, c| {
            if c.is_whitespace() {
                acc.push(String::new());
            } else {
                if acc.is_empty() {
                    acc.push(String::new());
                }
                acc.last_mut().unwrap().push(c);
            }
            acc
        });
    let mut freq: HashMap<String, i32> = HashMap::new();
    for word in words.into_iter() {
        if word.is_empty() || s.contains(&word) {
            continue;
        }
        *freq.entry(word).or_insert(0) += 1;
    }
    let largest = freq.into_iter().max_by_key(|e| e.1).unwrap();
    largest.0
}

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut mp: HashMap<String, Vec<String>> = HashMap::new();
    for w in strs.into_iter() {
        let c = w.clone();
        let mut sorted_c = c.chars().collect::<Vec<char>>();
        sorted_c.sort();
        let sorted = sorted_c.into_iter().collect::<String>();
        mp.entry(sorted).or_default().push(c);
    }
    Vec::from_iter(mp.into_values().into_iter().map(|v| Vec::from_iter(v)))
}

pub fn my_pow(x: f64, n: i32) -> f64 {
    if x == 0.0 {
        return 0.0;
    }
    if n == 0 {
        return 1.0;
    }
    let mut base = x;
    let mut res = 1.0;
    let mut exp = n as i64;
    if n < 0 {
        exp = -exp;
        base = 1.0 / base;
    }
    while exp != 0 {
        if exp % 2 == 1 {
            res *= base;
        }
        base *= base;
        exp /= 2;
    }
    res
}

pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    if n == 1 {
        return vec![vec!["Q".to_string()]];
    }
    let mask = (1 << n) - 1;
    let mut solutions: Vec<Vec<String>> = Vec::new();
    let mut board: Vec<usize> = Vec::new();
    fn solve(
        n: i32,
        row: i32,
        col: i32,
        ld: i32,
        rd: i32,
        mask: i32,
        board: &mut Vec<usize>,
        solutions: &mut Vec<Vec<String>>,
    ) {
        if row == n {
            let mut v: Vec<String> = Vec::with_capacity(n as usize);
            for &col in board.iter() {
                let mut row = vec![b'.'; n as usize];
                row[col] = b'Q';
                v.push(unsafe { String::from_utf8_unchecked(row) });
            }
            solutions.push(v);
            return;
        }
        let mut safe = !(col | ld | rd) & mask;
        while safe != 0 {
            let p = safe & -safe; // get the rightmost 1 as new placement
            let zeros = p.trailing_zeros();
            board.push(zeros as usize);
            // backtracking
            solve(
                n,
                row + 1,
                col | p,
                (ld | p) << 1,
                (rd | p) >> 1,
                mask,
                board,
                solutions,
            );
            safe &= safe - 1; // remove the rightmost 1
            board.pop();
        }
    }
    solve(n, 0, 0, 0, 0, mask, &mut board, &mut solutions);
    solutions
}

pub fn total_n_queens(n: i32) -> i32 {
    if n == 1 {
        return 1;
    }
    let mask = (1 << n) - 1;
    let mut cnt = 0;
    fn solve(n: i32, r: i32, c: i32, ld: i32, rd: i32, msk: i32, cnt: &mut i32) {
        if n == r {
            *cnt += 1;
            return;
        }
        let mut safe = !(c | ld | rd) & msk;
        while safe != 0 {
            let p = safe & -safe;
            solve(n, r + 1, c | p, (ld | p) << 1, (rd | p) >> 1, msk, cnt);
            safe &= safe - 1;
        }
        return;
    }
    solve(n, 0, 0, 0, 0, mask, &mut cnt);
    return cnt;
}
