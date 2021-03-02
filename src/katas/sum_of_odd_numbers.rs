#![allow(unused)]

/*
2 = 2 values (3, 5)
result = 3 + 5 = 8

***

value: an = a(n - 1) + 2;

***

2 = (3, 5) = 8 -> 1 + 2 * 2 = 5 (last element)
3 = (7, 9, 11) = 27 -> 2 + 3 * 3  = 11 (last element)
4 = (13, 15, 17, 19) -> 3 + 4 * 4 = 19 (last element)

for last element

an_max = (n - 1) + n * n

***

2 = (3, 5) = 8 -> (5 + 2) - (2 * 2) = 3 (first element)
3 = (7, 9, 11) = 27 -> (11 + 2) - (3 * 2) = 7 (first element)
4 = (13, 15, 17, 19) -> (19 + 2) - (4 * 2) = 13  (first element)

for first element

an_min = (an_max + 2) - (n * 2)

 */

fn row_sum_odd_numbers(n:i64) -> i64 {
    if n == 1 {
        return 1;
    }

    let last_element_value : i64 = (n - 1) + n * n;
    let first_element_value : i64 = (last_element_value + 2) - (n * 2);

    let mut result: i64 = 0;

    for v in (first_element_value..last_element_value).step_by(2) {
        result += v;
    }

    result += last_element_value;

    result
}

#[test]
fn returns_expected() {
    assert_eq!(row_sum_odd_numbers(1), 1);
    assert_eq!(row_sum_odd_numbers(5), 125);
    assert_eq!(row_sum_odd_numbers(42), 74088);
}

//Best solution is n.pow(3)
