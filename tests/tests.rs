
#[cfg(test)]
mod tests {
    use pathtracer::math::vec3::Vector3;
    use pathtracer::renderables::triangle::Triangle;
    use pathtracer::renderables::material::Material;

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
        Vector3::new(1., -1., 0.), Vector3::new(0., 1., 0.),
        None, Material::new([120, 50, 45], 1.));

        let normal_for_triangle = Vector3::new(0., 0., 1.);
        assert_eq!(triangle.get_normal().clone(), normal_for_triangle);
    }
}
