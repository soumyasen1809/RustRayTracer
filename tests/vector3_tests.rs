use lib::utilities::vector3::Vector3;

mod common_config;

#[test]
fn length_squared_test() {
    let vec_test = Vector3::new(1.0, 2.0, 3.0);
    assert_eq!(vec_test.length_squared(), 14.0);
}
