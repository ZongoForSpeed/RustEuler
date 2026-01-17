use crate::register_problem;
use crate::utils::permutations::permutations;
use string_builder::Builder;

register_problem!(68, "Magic 5-gon ring", problem068);

pub fn problem068() -> String {
    // Consider the following "magic" 3-gon ring, filled with the numbers 1 to 6, and each line
    // adding to nine.
    //
    // Working clockwise, and starting from the group of three with the numerically lowest external
    // node (4,3,2 in this example), each solution can be described uniquely. For example, the above
    // solution can be described by the set: 4,3,2; 6,2,1; 5,1,3.
    //
    // It is possible to complete the ring with four different totals: 9, 10, 11, and 12. There are
    // eight solutions in total.
    //
    // Total    Solution Set
    // 9        4,2,3; 5,3,1; 6,1,2
    // 9        4,3,2; 6,2,1; 5,1,3
    // 10       2,3,5; 4,5,1; 6,1,3
    // 10       2,5,3; 6,3,1; 4,1,5
    // 11       1,4,6; 3,6,2; 5,2,4
    // 11	    1,6,4; 5,4,2; 3,2,6
    // 12       1,5,6; 2,6,4; 3,4,5
    // 12       1,6,5; 3,5,4; 2,4,6
    //
    // By concatenating each group it is possible to form 9-digit strings; the maximum string for a
    // 3-gon ring is 432621513.
    //
    // Using the numbers 1 to 10, and depending on arrangements, it is possible to form 16- and
    // 17-digit strings. What is the maximum 16-digit string for a "magic" 5-gon ring?
    let arrangements = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut max_solution = 0;
    for permute in permutations(arrangements) {
        let k = permute[0] + permute[1] + permute[6];
        if permute[1] + permute[2] + permute[7] == k
            && permute[2] + permute[3] + permute[8] == k
            && permute[3] + permute[4] + permute[9] == k
            && permute[4] + permute[0] + permute[5] == k
            && permute[5] < permute[6]
            && permute[5] < permute[7]
            && permute[5] < permute[8]
            && permute[5] < permute[9]
        {
            let mut builder = Builder::default();

            builder.append(permute[5].to_string());
            builder.append(permute[0].to_string());
            builder.append(permute[4].to_string());
            builder.append(permute[9].to_string());
            builder.append(permute[4].to_string());
            builder.append(permute[3].to_string());
            builder.append(permute[8].to_string());
            builder.append(permute[3].to_string());
            builder.append(permute[2].to_string());
            builder.append(permute[7].to_string());
            builder.append(permute[2].to_string());
            builder.append(permute[1].to_string());
            builder.append(permute[6].to_string());
            builder.append(permute[1].to_string());
            builder.append(permute[0].to_string());

            if builder.len() == 16 {
                let solution = builder.string().unwrap().parse::<u64>().unwrap();
                println!("{:?} -> {:?}", permute, solution);
                max_solution = std::cmp::max(max_solution, solution);
            }
        }
    }

    max_solution.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem068() {
        let result = problem068();
        assert_eq!(result, "6531031914842725");
    }
}
