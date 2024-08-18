use lib::utilities::vector3::{Cross, Vector3};

mod common_config;

#[test]
fn length_squared_test() {
    let vec_test = Vector3::new(1.0, 2.0, 3.0);
    assert_eq!(vec_test.length_squared(), 14.0);
}

#[test]
fn cross_product_test() {
    let lhs_vec = Vector3::new(5.0, 7.0, 1.0);
    let rhs_vec = Vector3::new(4.0, 4.0, 3.0);
    let cross_prod_vec = lhs_vec.cross_prod(rhs_vec);

    assert_eq!(cross_prod_vec.get_x(), 17.0);
    assert_eq!(cross_prod_vec.get_y(), -11.0);
    assert_eq!(cross_prod_vec.get_z(), -8.0);
}

#[test]
fn dot_product_test() {
    let lhs_vec = Vector3::new(5.0, 7.0, 1.0);
    let rhs_vec = Vector3::new(4.0, 4.0, 3.0);
    let dot_prod = lhs_vec.dot_prod(rhs_vec);

    assert_eq!(dot_prod, 51.0);
}

#[test]
fn reflection_test() {
    let ray_vec = Vector3::new(5.0, 7.0, 1.0);
    let normal_vec = Vector3::new(1.0, 1.0, 1.0);
    let reflected_ray = ray_vec.reflection(&normal_vec);

    assert_eq!(reflected_ray.get_x(), -21.0);
    assert_eq!(reflected_ray.get_y(), -19.0);
    assert_eq!(reflected_ray.get_z(), -25.0);
}

#[test]
fn refraction_high_refractiveindex_test() {
    let ray_vec = Vector3::new(5.0, 7.0, 1.0);
    let normal_vec = Vector3::new(1.0, 1.0, 1.0);
    let ratio_refractive_index = 2.0;
    let refracted_ray = ray_vec.refraction(&normal_vec, ratio_refractive_index);

    assert_eq!(refracted_ray.get_x().trunc(), -47.0);
    assert_eq!(refracted_ray.get_y().trunc(), -43.0);
    assert_eq!(refracted_ray.get_z().trunc(), -55.0);
}

#[test]
fn refraction_low_refractiveindex_test() {
    let ray_vec = Vector3::new(5.0, 7.0, 1.0);
    let normal_vec = Vector3::new(1.0, 1.0, 1.0);
    let ratio_refractive_index = 1.0 / 2.0;
    let refracted_ray = ray_vec.refraction(&normal_vec, ratio_refractive_index);

    assert_eq!(refracted_ray.get_x().trunc(), -11.0);
    assert_eq!(refracted_ray.get_y().trunc(), -10.0);
    assert_eq!(refracted_ray.get_z().trunc(), -13.0);
}
