fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let vec = vec;

    vec.push(88);

    vec
}

fn main() {
    // You can optionally experiment here.
    let vec0 = vec![22, 44, 66];
    // 不會影響到原本的資料 但是會有額外的成本
    // 保留變數的數據跟所有權，需要保留原資料的時候可以用
    fill_vec(vec0.clone());
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Make both vectors `vec0` and `vec1` accessible at the same time to
    // fix the compiler error in the test.
    #[test]
    fn move_semantics2() {
        let vec0 = vec![22, 44, 66];

        // solution 1: use clone method
        let vec1 = fill_vec(vec0.clone());

        assert_eq!(vec0, [22, 44, 66]);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
