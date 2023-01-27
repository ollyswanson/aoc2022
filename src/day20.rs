pub fn run(input: &str) -> anyhow::Result<(i64, i64)> {
    let input: Vec<i64> = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<_, _>>()?;

    let part_1 = mix(&input, 1, 1);
    let part_2 = mix(&input, 10, 811589153);

    Ok((part_1, part_2))
}

fn mix(nums: &[i64], rounds: usize, key: i64) -> i64 {
    let nums = nums.iter().map(|x| x * key).collect::<Vec<_>>();
    let mut ans = (0..nums.len()).collect::<Vec<_>>();
    for _ in 0..rounds {
        for (i, &x) in nums.iter().enumerate() {
            let pos = ans.iter().position(|&y| y == i).unwrap();
            ans.remove(pos);
            let new_i = (pos as i64 + x).rem_euclid(ans.len() as i64) as usize;
            ans.insert(new_i, i);
        }
    }
    let i1 = nums.iter().position(|&i| i == 0).unwrap();
    let i2 = ans.iter().position(|&i| i == i1).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|i| nums[ans[(i2 + i) % ans.len()]])
        .sum()
}
