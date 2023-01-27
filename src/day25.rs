pub fn run(input: &str) -> anyhow::Result<(String, u32)> {
    let sum = input.lines().map(parse_snafu).sum::<Result<i64, _>>()?;

    println!("{}", sum);

    let snafu = decimal_to_snafu(sum);

    Ok((snafu, 0))
}

fn parse_snafu(snafu: &str) -> anyhow::Result<i64> {
    snafu.bytes().rev().enumerate().try_fold(0, |acc, (i, b)| {
        let fives = 5i64.pow(i as u32);

        let figit = match b {
            b'0' => 0,
            b'1' => 1,
            b'2' => 2,
            b'-' => -1,
            b'=' => -2,
            _ => anyhow::bail!("unrecognized figit"),
        };

        Ok(acc + fives * figit)
    })
}

fn decimal_to_snafu(mut num: i64) -> String {
    let mut snafu = Vec::new();
    let figits = [b'=', b'-', b'0', b'1', b'2'];

    while num > 0 {
        let place = (num + 2).rem_euclid(5);
        snafu.push(figits[place as usize]);
        num = (num + 2) / 5;
    }

    snafu.reverse();

    // Safety: We know that we have constructed the string with valid utf-8
    unsafe { String::from_utf8_unchecked(snafu) }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "\
1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    #[test]
    fn snafu_parser_works() {
        let sum = TEST_INPUT
            .lines()
            .map(parse_snafu)
            .sum::<Result<i64, _>>()
            .unwrap();

        assert_eq!(4890, sum);
    }

    #[test]
    fn decimal_to_snafu_test() {
        let decimal = 4890;
        let snafu = "2=-1=0";

        assert_eq!(snafu, decimal_to_snafu(decimal))
    }
}
