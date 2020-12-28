use algorithms_rust;

#[test]
fn it_variance() {
    // let input: [f64; 4] = [5.0, 8.0, 10.0, 7.0];
    let input = [5.0, 8.0, 10.0, 7.0];

    assert_eq!(
        3.25,
        algorithms_rust::statistics::descriptive::variance(&input)
    );
}

#[test]
fn it_std_dev() {
    // let input: [f64; 4] = [5.0, 8.0, 10.0, 7.0];
    let input = [5.0, 8.0, 10.0, 7.0];

    assert_eq!(
        1.8027756377319946,
        algorithms_rust::statistics::descriptive::std_dev(&input)
    );
}
