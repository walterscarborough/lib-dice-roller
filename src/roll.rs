use rand::Rng;

pub fn roll(size: i32, count: i32) -> Vec<i32> {

    let mut output: Vec<i32> = vec![];

    let max_inclusive_size = size + 1;
    let mut rng = rand::thread_rng();

    for _ in 0..count {
        let random_number = rng.gen_range(1, max_inclusive_size);
        output.push(random_number);
    }

    return output;
}

#[cfg(test)]
mod tests {
    use crate::roll::roll;
    use std::borrow::Borrow;

    #[test]
    fn roll_should_return_random_values() {
        let result_a= roll(6, 2000);
        let result_b= roll(6, 2000);

        assert_ne!(result_a, result_b);
    }

    #[test]
    fn roll_should_return_results_within_the_specified_size() {
        let result= roll(6, 2000);

        for value in &result {
            assert!(value.to_owned() >= 1, "should be greater than or equal to 1");
            assert!(value.to_owned() <= 6, "should be less than or equal to 6");
        }
    }

    #[test]
    fn roll_should_return_the_result_count_that_was_requested() {
        let result= roll(6, 2000);

        assert_eq!(result.len(), 2000);
    }

    #[test]
    fn roll_should_return_rolls_of_varying_size() {
        let result= roll(20, 1);

        for value in &result {
            assert!(value.to_owned() >= 1, "should be greater than or equal to 1");
            assert!(value.to_owned() <= 20, "should be less than or equal to 20");
        }
    }

    #[test]
    fn roll_should_be_able_to_reach_one() {
        let result= roll(6, 1000);

        let mut reached_one = false;
        for value in &result {
            if value.eq(1i32.borrow()) {
                reached_one = true;
                break;
            }
        }

        assert_eq!(reached_one, true);
    }

    #[test]
    fn roll_should_be_able_to_reach_max_size() {
        let result= roll(6, 1000);

        let mut reached_max_size = false;
        for value in &result {
            if value.eq(6i32.borrow()) {
                reached_max_size = true;
                break;
            }
        }

        assert_eq!(reached_max_size, true);
    }
}
