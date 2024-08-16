use lib::utilities::color::{Color, Cross};

mod common_config;

#[test]
fn add_assign_test() {
    let mut color_test = Color::new(1.0, 2.0, 3.0);
    color_test += Color::new(4.0, 0.0, 6.0);
    assert_eq!(color_test.get_r(), 5.0);
    assert_eq!(color_test.get_g(), 2.0);
    assert_eq!(color_test.get_b(), 9.0);
}

#[test]
fn multiply_assign_test() {
    let mut color_test = Color::new(10.0, 20.0, 30.0);
    color_test *= Color::new(4.0, 0.0, 6.0);
    assert_eq!(color_test.get_r(), 40.0);
    assert_eq!(color_test.get_g(), 0.0);
    assert_eq!(color_test.get_b(), 180.0);
}

#[test]
fn cross_product_test() {
    let lhs_color = Color::new(5.0, 7.0, 1.0);
    let rhs_color = Color::new(4.0, 4.0, 3.0);
    let cross_prod_col = lhs_color.cross_prod(rhs_color);

    assert_eq!(cross_prod_col.get_r(), 17.0);
    assert_eq!(cross_prod_col.get_g(), -11.0);
    assert_eq!(cross_prod_col.get_b(), -8.0);
}
