mod declaration;
mod solutions;
fn main() {
    let mat: Vec<Vec<i32>> = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
    let s = solutions::spiral_order(mat);
}
