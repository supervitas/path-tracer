
#[cfg(test)]
mod tests {
    use pathtracer::math::vec3::Vector3;
    use pathtracer::renderables::triangle::Triangle;
    use pathtracer::renderables::material::Material;
    use pathtracer::math::color::Color;
    use std::ops::Mul;
    use pathtracer::math::mat4::Matrix4;

    #[test]
    fn cross_product() {
        let mut v1 = Vector3::new(1., 2., 3.);
        let v2 = Vector3::new(1., 5., 7.);
        v1.cross(&v2);

        let result = Vector3::new(-1., -4., 3.);
        assert_eq!(v1, result);
    }

    #[test]
    fn create_triangle() {
        let triangle = Triangle::new(Vector3::new(-1., -1., 0.),
        Vector3::new(1., -1., 0.), Vector3::new(0., 1., 0.));

        let normal_for_triangle = Vector3::new(0., 0., 1.);
        assert_eq!(triangle.get_normal().clone(), normal_for_triangle);
    }

    #[test]
    fn angle_between_two_v() {
        let v1 = Vector3::new(10., 30., 40.);
        let v2 = Vector3::new(20., 3., 4.);

        assert_eq!(1.1284221038181517, v1.angle_between(&v2));
    }

    #[test]
    fn look_at() {
        let origin = Vector3::new(0.,5.,0.);
        let target = Vector3::new(0.,0.,0.);

        let mat = Matrix4::look_at(&origin, &target);

        let mut etalon = Matrix4::from_array([0.9999999999999999, 0., 0., 0., 0., 0.0000999999995, -0.9999999949999999, 0., 0., 0.999999995, 0.00009999999950000001, 0., 0., 0., 0., 1.]);


        assert_eq!(mat, etalon);
    }

    #[test]
    fn matrix_mul() {
        let mut mat = Matrix4::new([
            8.,1.,3.,2.,
            5.,1.,3.,5.,
            6.,7.,13.,20.,
            23.,32.,5.,10.
        ]);

        let second_mat = Matrix4::new([
            5.,1.,3.,4.,
            5.,1.,3.,5.,
            6.,7.,10.,20.,
            30.,40.,50.,60.
        ]);

        mat.multiply(&second_mat);

        let mut etalon = Matrix4::from_array([123., 198., 743., 605., 110., 227., 904., 490., 157., 298., 1169., 715., 217., 385., 1519., 952.]);

        assert_eq!(mat, etalon);
    }

    #[test]
    fn test_color_mul() {
        let mut color = Color::new(0.5, 1., 0.3);
        color = color * 2.;

        let etalon = Color::new(1., 2., 0.6);
        assert_eq!(color, etalon);
    }
}
