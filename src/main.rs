mod declaration;
mod solutions;
fn main() {
    let s = solutions::get_permutation(4, 9);
    println!("{}", s);
}
// /**
//  * 1234
//  * 1243
//  * 1324
//  * 1342
//  * 1423
//  * 1432
//  * 2134
//  * 2143
//  * 2314
//  * 2341
//  * 2413
//  * 2431
//  */
