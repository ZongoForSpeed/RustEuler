use crate::register_problem;
use permutohedron::LexicalPermutation;
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
    let mut arrangements = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut max_solution = 0;
    loop {
        let k = arrangements[0] + arrangements[1] + arrangements[6];
            if arrangements[1] + arrangements[2] + arrangements[7] == k
            && arrangements[2] + arrangements[3] + arrangements[8] == k
            && arrangements[3] + arrangements[4] + arrangements[9] == k
            && arrangements[4] + arrangements[0] + arrangements[5] == k
            && arrangements[5] < arrangements[6]
            && arrangements[5] < arrangements[7]
            && arrangements[5] < arrangements[8]
            && arrangements[5] < arrangements[9] {
                let mut builder = Builder::default();

                builder.append(arrangements[5].to_string());
                builder.append(arrangements[0].to_string());
                builder.append(arrangements[4].to_string());
                builder.append(arrangements[9].to_string());
                builder.append(arrangements[4].to_string());
                builder.append(arrangements[3].to_string());
                builder.append(arrangements[8].to_string());
                builder.append(arrangements[3].to_string());
                builder.append(arrangements[2].to_string());
                builder.append(arrangements[7].to_string());
                builder.append(arrangements[2].to_string());
                builder.append(arrangements[1].to_string());
                builder.append(arrangements[6].to_string());
                builder.append(arrangements[1].to_string());
                builder.append(arrangements[0].to_string());

                if builder.len() == 16 {
                    let solution = builder.string().unwrap().parse::<u64>().unwrap();
                    println!("{:?} -> {:?}", arrangements, solution);
                    max_solution = std::cmp::max(max_solution, solution);
                }

            }

        if !arrangements.next_permutation() {
            break;
        }
    }

    max_solution.to_string()
}
