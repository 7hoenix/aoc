pub fn part_one(input: &str) -> Option<u32> {
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

    let results: Option<u32> = elf_inputs.iter().map(|elf| elf.iter().sum()).max();

    // let elf_list: Vec<Vec<u32>> =
    //     raw_inputs.iter()
    //     .ma

    // let inputs: Vec<u32> = raw_inputs
    //     .iter()
    //     .map(|x| x.parse().expect("Not an integer"))
    //     .collect();
    // let processed: u32 = inputs.iter().sum();
    // println!("{:?}", processed);
    // // input.split('\n').map(str.);
    // let my_int: Result<u32, core::num::ParseIntError> = input.parse();
    // my_int.ok()
    results
}

// fn get_elf(stream: Vec<&str>) -> (Vec<&str>, Vec<&str>) {
//     stream.iter().map()
//     ([], [])
// }

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_one(&input), Some(0));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
