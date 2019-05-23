mod math;

fn main() {
    let camera_position = math::vec3::Vector3{x:2, y:2, z: 5};
    let camera_position2 = math::vec3::Vector3{x:2, y:1, z: 15};

    let t = camera_position.dot(camera_position2);

    println!("{}", t);
}
