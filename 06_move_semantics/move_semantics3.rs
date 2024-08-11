// TODO: Fix the compiler error in the function without adding any new line.
fn fill_vec(vec: &mut Vec<i32>) -> &mut Vec<i32> {
    vec.push(88);

    vec
}

fn main() {
    // You can optionally experiment here.
    let mut vec0 = vec![22, 44, 66];
    // 借用所有權修改同一組資料
    // 如果不需要保留原本的資料可以使用這個方法
    fill_vec(&mut vec0);
}

#[cfg(test)]
mod tests {
    use super::*;

    // solution 2 : use reference 
    #[test]
    fn move_semantics3() {
        let mut vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(&mut vec0);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
