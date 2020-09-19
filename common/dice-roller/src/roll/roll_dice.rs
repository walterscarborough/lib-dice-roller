use crate::roll::roll_request::RollRequest;
use crate::roll::roll_response::RollResponse;
use rand::Rng;

pub fn roll_dice(roll_request: RollRequest) -> RollResponse {
    let mut output: Vec<u32> = vec![];

    let inclusive_dice_size_upper_bound = roll_request.dice_size + 1;
    let mut rng = rand::thread_rng();

    for _ in 0..roll_request.number_of_rolls {
        let random_number = rng.gen_range(1, inclusive_dice_size_upper_bound);
        output.push(random_number);
    }

    RollResponse {
        dice_values: output,
    }
}

#[cfg(test)]
mod tests {
    use crate::roll::roll_dice::roll_dice;
    use crate::roll::roll_request::RollRequest;
    use std::borrow::Borrow;

    #[test]
    fn roll_should_return_random_values() {
        let roll_request = RollRequest {
            dice_size: 6,
            number_of_rolls: 2000,
        };

        let roll_response_a = roll_dice(roll_request);
        let roll_response_b = roll_dice(roll_request);

        assert_ne!(roll_response_a, roll_response_b);
    }

    #[test]
    fn roll_should_return_results_within_the_specified_size() {
        let roll_request = RollRequest {
            dice_size: 6,
            number_of_rolls: 2000,
        };

        let roll_response = roll_dice(roll_request);

        for dice_value in roll_response.dice_values {
            assert!(
                dice_value.to_owned() >= 1,
                "should be greater than or equal to 1"
            );
            assert!(
                dice_value.to_owned() <= 6,
                "should be less than or equal to 6"
            );
        }
    }

    #[test]
    fn roll_should_return_the_result_count_that_was_requested() {
        let roll_request = RollRequest {
            dice_size: 6,
            number_of_rolls: 2000,
        };

        let roll_response = roll_dice(roll_request);

        assert_eq!(roll_response.dice_values.len(), 2000);
    }

    #[test]
    fn roll_should_return_rolls_of_varying_dice_size() {
        let roll_request = RollRequest {
            dice_size: 20,
            number_of_rolls: 2000,
        };

        let roll_response = roll_dice(roll_request);

        for dice_value in roll_response.dice_values {
            assert!(
                dice_value.to_owned() >= 1,
                "should be greater than or equal to 1"
            );
            assert!(
                dice_value.to_owned() <= 20,
                "should be less than or equal to 20"
            );
        }
    }

    #[test]
    fn roll_should_be_able_to_reach_dice_value_of_one() {
        let roll_request = RollRequest {
            dice_size: 6,
            number_of_rolls: 2000,
        };

        let roll_response = roll_dice(roll_request);

        let mut reached_one = false;
        for dice_value in roll_response.dice_values {
            if dice_value.eq(1u32.borrow()) {
                reached_one = true;
                break;
            }
        }

        assert_eq!(reached_one, true);
    }

    #[test]
    fn roll_should_be_able_to_reach_max_dice_size() {
        let roll_request = RollRequest {
            dice_size: 6,
            number_of_rolls: 2000,
        };

        let roll_response = roll_dice(roll_request);

        let mut reached_max_size = false;
        for dice_value in roll_response.dice_values {
            if dice_value.eq(6u32.borrow()) {
                reached_max_size = true;
                break;
            }
        }

        assert_eq!(reached_max_size, true);
    }
}
