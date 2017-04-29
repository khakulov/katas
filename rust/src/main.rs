fn main() {
    println!("{}", calculate(vec![1,2,3]));
}

// fn findBestWeekWindow(vec: Vec<i32>, i: i32) -> usize {
// 
//     return -1;
// }

fn calculate(vec: Vec<i32>) -> usize {
    let current_sum = vec.len() * 2;
    
    if vec.len() < 4 {
        return current_sum;
    }
    
    // for (i = 0; i < vec.len(); i++) {
    //     let j: i32 = findBestWeekWindow(A, i);
    //     print("{}", j);
    //     // if j != -1 {
    //     //     const new_prise = calculate(vec[0..i] + vec[j..vec.len()])
    //     // }
    // }
    // 
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


#[test]
fn for_loop() {
    let v = vec!["1", "2", "3"];
    for x in v {
        println!("{}", x);
    }
    assert_eq!(1, 1);
}


#[test]
fn for_loop_iterate_twice() {
    let v = vec!["1", "2", "3"];
    for x in &v {
        println!("{}", x);
    }
    for x in &v {
        println!("{}", x);
    }
    assert_eq!(1, 1);
}