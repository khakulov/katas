fn main() {
    println!("{}", calculate(vec![1,2,3]));
}

fn calculate(vec: Vec<i32>) -> usize {
    let current_sum = vec.len() * 2;

    if vec.len() < 4 {
        return current_sum;
    }

    current_sum
}

#[test]
fn return_0_for_empty_list() {
    assert_eq!(calculate(vec![]), 0);
}

#[test]
fn return_2_for_list_with_one_element() {
    assert_eq!(calculate(vec![1]), 2);
}

#[test]
fn return_6_for_list_with_three_element() {
    assert_eq!(calculate(vec![1,2,3]), 6);
}

#[test]
fn return_7_for_list_with_days_which_fit_into_one_week() {
    assert_eq!(calculate(vec![1,2,3,4]), 7);
}

#[test]
fn return_9_for_list_with_days_which_fit_into_one_week_and_extra_day() {
    assert_eq!(calculate(vec![1,2,5,7]), 7);
}