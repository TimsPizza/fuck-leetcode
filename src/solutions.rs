use std::{
    cmp::max,
    collections::{self, HashMap, HashSet},
    i32::MAX,
    num, string,
    usize::MIN,
    vec,
};

use crate::declaration::*;

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

pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }
    let mut local = nums[0];
    let mut mx = nums[0];
    for i in 1..nums.len() {
        local = max(local + nums[i], nums[i]);
        mx = max(local, mx);
    }
    mx
}

pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut ans: Vec<i32> = Vec::with_capacity(rows * cols);
    let (mut top, mut left, mut bottom, mut right) = (0, 0, rows, cols);
    while top < bottom && left < right {
        // top row
        for i in left..right {
            ans.push(matrix[top][i]);
        }
        top += 1;
        if top >= bottom {
            break;
        }
        // right col
        for i in top..bottom {
            ans.push(matrix[i][right]);
        }
        right -= 1;
        if right <= left {
            break;
        }
        for i in (left..right).rev() {
            ans.push(matrix[bottom][i]);
        }
        bottom -= 1;
        if bottom <= top {
            break;
        }
        // left col
        for i in (top..bottom).rev() {
            ans.push(matrix[i][left]);
        }
        left += 1;
        if left >= right {
            break;
        }
    }
    ans
}

pub fn can_jump(nums: Vec<i32>) -> bool {
    if nums.len() == 1 {
        return true;
    }
    if nums[0] == 0 {
        return false;
    }
    let mut mx = 0;
    let mut local = nums[0];
    for i in 1..nums.len() {
        if i as i32 <= local {
            local = local.max(nums[i] + i as i32);
        }
        mx = mx.max(local);
    }
    mx >= (nums.len() - 1) as i32
}

pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut ans: Vec<Vec<i32>> = Vec::new();
    if intervals.len() == 1 {
        return intervals;
    }
    let mut lo: i32;
    let mut hi: i32;
    intervals.sort_by_key(|pair| pair[0]);
    ans.push(vec![intervals[0][0], intervals[0][1]]);
    for i in 1..intervals.len() {
        lo = intervals[i][0];
        hi = intervals[i][1];
        if lo > ans.last().unwrap()[1] {
            ans.push(intervals[i].clone());
        } else {
            let last_interval = ans.last_mut().unwrap();
            last_interval[1] = last_interval[1].max(hi);
        }
    }
    ans
}

pub fn insert(intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let mut i = 0usize;
    let n = intervals.len();
    let mut ans: Vec<Vec<i32>> = Vec::new();
    while i < n && new_interval[0] > intervals[i][1] {
        ans.push(intervals[i].clone());
        i += 1;
    }
    while i < n && new_interval[1] >= intervals[i][0] {
        new_interval[0] = new_interval[0].min(intervals[i][0]);
        new_interval[1] = new_interval[1].max(intervals[i][1]);
        i += 1;
    }
    ans.push(new_interval);
    while i < n {
        ans.push(intervals[i].clone());
        i += 1;
    }
    ans
}

pub fn length_of_last_word(s: String) -> i32 {
    let mut i = 0;
    let it_c = s.trim().chars().into_iter();
    for c in it_c.rev() {
        if c.is_whitespace() {
            break;
        }
        i += 1;
    }
    i
}

pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    let mut ans: Vec<Vec<i32>> = vec![vec!(0; n as usize); n as usize];
    let mut num = 1;
    let (mut t, mut l, mut r, mut b) = (0usize, 0usize, n as usize, n as usize);
    while t < b && l < r {
        // top row
        for i in l..r {
            ans[t][i] = num;
            num += 1;
        }
        t += 1;
        for i in t..b {
            ans[i][r - 1] = num;
            num += 1;
        }
        r -= 1;
        for i in (l..r).rev() {
            ans[b - 1][i] = num;
            num += 1;
        }
        b -= 1;
        for i in (t..b).rev() {
            ans[i][l] = num;
            num += 1;
        }
        l += 1
    }

    ans
}

pub fn get_permutation(n: i32, k: i32) -> String {
    let fac: Vec<u32> = vec![1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];
    let mut available: Vec<u32> = (1..n as u32 + 1).collect();
    let mut ans: Vec<char> = Vec::new();
    fn solve(
        n: i32,
        digit: u32,
        round: u32,
        available: &mut Vec<u32>,
        fac: &Vec<u32>,
        ans: &mut Vec<char>,
    ) {
        if digit == n as u32 {
            return;
        }
        let base = fac[available.len() - 1] as u32;
        let offset = (round / base) as usize;
        let d = available.remove(offset);
        ans.push(char::from_digit(d, 10).unwrap());
        solve(
            n,
            digit + 1,
            round - offset as u32 * base,
            available,
            fac,
            ans,
        )
    }
    solve(n, 0, (k as u32) - 1, &mut available, &fac, &mut ans);
    ans.iter().collect()
}
