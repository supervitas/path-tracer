
#[cfg(test)]
mod tests {
    use pathtracer::math::vec3::Vector3;
    #[test]
    fn cross_product() {
        let mut v1 = Vector3::new(1., 2., 3.);
        let v2 = Vector3::new(1., 5., 7.);
        v1.cross(&v2);

        let result = Vector3::new(-1., -4., 3.);
        assert_eq!(v1, result);
    }
}
