use lib::utilities::color::Color;

mod common_config;

#[test]
fn add_assign_test() {
    let mut color_test = Color::new(1.0, 2.0, 3.0);
    color_test += Color::new(4.0, 0.0, 6.0);
    assert_eq!(color_test.get_r(), 5.0);
    assert_eq!(color_test.get_g(), 2.0);
    assert_eq!(color_test.get_b(), 9.0);
}
