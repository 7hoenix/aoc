pub fn part_one(input: &str) -> Option<u32> {
    sorted_list(&input).pop()
}

fn sorted_list(input: &str) -> Vec<u32> {
    let raw_inputs: Vec<&str> = input
        .split("\n\n")
        // .map(|elf| {
        //     elf.split("\n")
        //         .map(|calorie| calorie.parse().expect("Not an integer"))
        //         .collect()
        // })
        // .map(|x| println!("{}", x); x)
        // .map(|calorie| x.parse().expect("Not an integer"))
        .collect();

    let elf_inputs: Vec<Vec<u32>> = raw_inputs
        .iter()
        .map(|elf| {
            elf.split("\n")
                .map(|calorie| calorie.parse())
                .filter_map(|r| r.ok())
                .collect()
        })
        .collect();

    // let printable: Vec<u32> = elf_inputs.iter().map(|elf| elf.iter().sum()).collect();

    let mut results: Vec<u32> = elf_inputs.iter().map(|elf| elf.iter().sum()).collect();

    results.sort();
    results
    // let mut slice: Vec<u32> = results.as_mut();

    // slice.sort()
}

// fn get_elf(stream: Vec<&str>) -> (Vec<&str>, Vec<&str>) {
//     stream.iter().map()
//     ([], [])
// }

pub fn part_two(input: &str) -> Option<u32> {
    Some(sorted_list(&input).iter().rev().take(3).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));

        let input = advent_of_code::read_file("inputs", 1);
        assert_eq!(part_one(&input), Some(69836));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));

        let input = advent_of_code::read_file("inputs", 1);
        assert_eq!(part_two(&input), Some(207968));
    }
}
