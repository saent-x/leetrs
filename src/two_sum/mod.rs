use std::collections::HashMap;

/// My initial solution
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // APPLY CONSTRAINTS
    // 2 <= nums.length <= 10^4 - done
    // -10^9 <= nums[i] <= 10^9 - done
    // -10^9 <= target <= 10^9 - done

    // first pick the first value - done
    // then loop throw the first value and all value in the array - done
    // if their are not matched move to the next value in the array - done
    // if none matches return an empty array else add the indices to the array and return it

    if 2 <= nums.len()
        && nums.len() <= 10_usize.pow(4)
        && -10_i32.pow(9) <= target
        && target <= 10_i32.pow(9)
    {
        for (i, &x) in nums.iter().enumerate() {
            println!("x -> {x}");

            if -10_i32.pow(9) <= x && x <= 10_i32.pow(9) {
                for (j, &y) in nums.iter().enumerate() {
                    println!("x[{x}] + y[{y}] -> {} | {target}", x + y);

                    if i != j && x + y == target {
                        println!("yay!! {i} - {j}");
                        return vec![i as i32, j as i32];
                    }
                }
            }
        }
    }

    vec![]
}

/// Best solution after review
pub fn two_sum_hm(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hm = HashMap::new();

    // first pass
    // lets iterate through all the value and store the diff and index in a hashmap
    for (i, num) in nums.iter().enumerate() {
        hm.insert(target - num, i);
    }

    // If we can find any value in the array as a key is the hashmap...
    // and the value its not same as the current index
    // then the value is the second value for the sum, since only complementing sum values exist in the hashmap
    for (i, num) in nums.iter().enumerate() {
        let second_idx = hm.get(num);
        if let Some(value) = second_idx
            && i != value.clone()
        {
            return vec![i as i32, value.to_owned() as i32];
        }
    }

    vec![]
}
