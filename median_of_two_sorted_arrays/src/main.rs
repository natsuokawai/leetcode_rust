fn main() {
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
        2.0
    );
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
        2.5
    );
    assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![1]), 1.0)
}

struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut id1 = 0;
        let mut id2 = 0;
        let mut result = Vec::new();

        while id1 < nums1.len() || id2 < nums2.len() {
            if id1 == nums1.len() && id2 == nums2.len() {
                break;
            }
            if id1 >= nums1.len() && id2 < nums2.len() {
                result.push(nums2[id2]);
                id2 += 1;
                continue;
            }
            if id1 < nums1.len() && id2 >= nums2.len() {
                result.push(nums1[id1]);
                id1 += 1;
                continue;
            }
            if nums1[id1] < nums2[id2] {
                result.push(nums1[id1]);
                id1 += 1;
            } else {
                result.push(nums2[id2]);
                id2 += 1
            }
        }

        if result.len() % 2 == 0 {
            (result[result.len() / 2 - 1] + result[result.len() / 2]) as f64 / 2.0
        } else {
            result[result.len() / 2] as f64
        }
    }
}
