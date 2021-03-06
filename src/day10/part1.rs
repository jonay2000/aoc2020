use std::collections::HashMap;

fn part1(inp: &str) -> Result<i64, ()> {
    let mut nums = inp.lines().filter(|i| !i.trim().is_empty()).map(|i| i.parse().unwrap()).collect::<Vec<i64>>();

    nums.insert(0, 0);

    nums.sort();
    let (ones, threes) = nums.windows(2).fold((0, 0), |(ones, threes), win| {
        match win[1] - win[0]{
            1 => (ones + 1, threes),
            3 => (ones, threes + 1),
            _ => (ones, threes),
        }
    });

    Ok(ones * (threes + 1))
}

fn count(nums: &[i64], curr: i64, max: i64, cache: &mut HashMap<i64, i64>) -> i64 {
    if let Some(i) = cache.get(&curr) {
        *i
    } else if curr == max {
        1
    } else {
         let res = nums.iter()
            .filter(|&&i| (1..=3).contains(&(i - curr)))
            .map(|i| count(nums, *i, max, cache))
            .sum();
        
        cache.insert(curr, res);

        res
    }
}


fn part2(inp: &str) -> Result<i64, ()> {
    let nums = inp.lines().filter(|i| !i.trim().is_empty()).map(|i| i.parse().unwrap()).collect::<Vec<i64>>();

    let mut cache = HashMap::new();
    Ok(count(&nums, 0, *nums.iter().max().unwrap(), &mut cache))
}



pub fn main() {  
    println!("day 10 part 1: {:?}", part1(include_str!("input1")));
    println!("day 10 part 2: {:?}", part2(include_str!("input1")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        assert_eq!(35, part1("
16
10
15
5
1
11
7
19
6
12
4
        ").unwrap())
    }

#[test]
    fn test_example_part2() {
        assert_eq!(8, part2("
16
10
15
5
1
11
7
19
6
12
4
        ").unwrap())
    }


    #[test]
    fn test_part1() {
        assert_eq!(1820, part1(include_str!("input1")).unwrap());
    }

    #[test]
    fn test_part2() {
        assert_eq!(3454189699072, part2(include_str!("input1")).unwrap());
    }
}



