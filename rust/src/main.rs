fn main() {
    println!("{}", calculate(vec![1,2,3]));
}

// fn findBestWeekWindow(vec: Vec<i32>, i: i32) -> usize {
// 
//     return -1;
// }

fn calculate(vec: Vec<i32>) -> usize {
    if vec.len() == 0 {
        return 0;
    }

    let current_sum = vec.len() * 2;
    
    
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
fn return_0_if_no_days_given() {
  assert_eq!(calculate(vec![]), 0);
}

#[test]
fn return_2_for_one_day() {
  assert_eq!(calculate(vec![1]), 2);
}

#[test]
fn return_6_for_three_days() {
  assert_eq!(calculate(vec![1,2,3]), 6);
}

#[test]
fn return_7_for_4_days_in_row() {
  assert_eq!(calculate(vec![1,2,3,4]), 7);
}

#[test]
fn return_7_for_all_days_in_one_week() {
  assert_eq!(calculate(vec![1,2,3,4]), 7);
  assert_eq!(calculate(vec![1,2,3,4,5]), 7);
  assert_eq!(calculate(vec![1,2,3,4,5,6]), 7);
  assert_eq!(calculate(vec![1,2,3,4,5,6,7]), 7);
  assert_eq!(calculate(vec![2,3,4,5,6,7]), 7);
  assert_eq!(calculate(vec![2,3,4,6,7]), 7);
  assert_eq!(calculate(vec![1,3,5,7]), 7);
}

#[test]
fn return_9_for_1_week_and_1_day() {
  assert_eq!(calculate(vec![1,2,3,4,8]), 9);
  assert_eq!(calculate(vec![1,2,3,4,5,8]), 9);
  assert_eq!(calculate(vec![1,2,3,4,5,6,8]), 9);
  assert_eq!(calculate(vec![1,2,3,4,5,6,7,8]), 9);
  assert_eq!(calculate(vec![1,2,3,4,5,6,8]), 9);
  assert_eq!(calculate(vec![1,2,3,4,6,8]), 9);
  assert_eq!(calculate(vec![1,2,3,5,8]), 9);
}

#[test]
fn return_7_and_extra_for_days_in_one_week_and_extra_days() {
    assert_eq!(calculate(vec![1,2,3,4,8,9]), 11);
    assert_eq!(calculate(vec![1,2,3,4,8,9,10]), 13);
    assert_eq!(calculate(vec![1,2,3,4,8,9,10,11]), 14);
}

#[test]
fn return_18_for_2_weeks_and_2_days() {
    assert_eq!(calculate(vec![1,5,6,7,8,9,10,11,12,16,17,18,19,20]), 18);
}

#[test]
fn return_25_for_one_month() {
    assert_eq!(calculate(vec![1,3,5,7,9,11,13,15,17,19,21,23,25,27,29]), 25);
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