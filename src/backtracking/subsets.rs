pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    struct Backtracking<'a> {
        res: Vec<Vec<i32>>,
        path: Vec<i32>,
        run: &'a dyn Fn(&mut Backtracking, usize),
    }

    let mut backtracking = Backtracking {
        res: vec![],
        path: vec![],
        run: &|bt, start| {
            bt.res.push(bt.path.clone());

            if start == nums.len() {
                return;
            }

            for i in start..nums.len() {
                bt.path.push(nums[i]);
                (bt.run)(bt, i + 1);
                bt.path.pop();
            }
        },
    };

    (backtracking.run)(&mut backtracking, 0);
    backtracking.res
}

#[cfg(test)]
mod tests {
    use super::subsets;

    #[test]
    fn subsets_it_works() {
        let input = vec![1, 2, 3];
        let output = vec![
            vec![],
            vec![1],
            vec![1, 2],
            vec![1, 2, 3],
            vec![1, 3],
            vec![2],
            vec![2, 3],
            vec![3],
        ];

        assert_eq!(subsets(input), output);
    }
}
