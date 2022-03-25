use crate::*;

pub struct Sphere {
    pub origin: Vec3,
    pub radius: f32
}

impl Sphere {
    pub fn new(origin: Vec3, radius: f32) -> Sphere {
        Sphere {
            origin,
            radius
        }
    }

    pub fn is_point_inside(&self, point: &Vec3) -> bool {
        let length_vector = (point - &self.origin).squared_magnitude();
        return length_vector <= self.radius * self.radius;
    }

    pub fn does_intersect_with_sphere(&self, other: &Sphere) -> bool {
        let result = &other.origin - &self.origin;
        return (result).magnitude() < (self.radius + other.radius);
    }
}

#[cfg(test)]
mod sphere_tests {
    use crate::*;

    #[test]
    fn point_sphere_intersection_tests() {
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 5.0);
        let origin_point = Vec3::new(0.0, 0.0, 0.0);
        let between_origin_and_shell = Vec3::new(0.0, 2.5, 0.0);
        let on_shell = Vec3::new(0.0, 0.0, 5.0);
        let beyond_shell = Vec3::new(0.0, 0.0, 5.1);

        assert_eq!(true, sphere.is_point_inside(&origin_point));
        assert_eq!(true, sphere.is_point_inside(&between_origin_and_shell));
        assert_eq!(true, sphere.is_point_inside(&on_shell));
        assert_eq!(false, sphere.is_point_inside(&beyond_shell));
    }

    #[test]
    fn sphere_sphere_intersection_simple_test()  {
        let sphere_a = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 5.0);
        let sphere_b = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0);
        assert_eq!(true, sphere_a.does_intersect_with_sphere(&sphere_b));
    }

    #[test]
    fn sphere_sphere_intersection_on_shell_test()  {
        let sphere_a = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 5.0);
        let mut sphere_b = Sphere::new(Vec3::new(0.0, 5.0, 0.0), 1.0);
        assert_eq!(true, sphere_a.does_intersect_with_sphere(&sphere_b));
        sphere_b.radius = 0.0;
        assert_eq!(false, sphere_a.does_intersect_with_sphere(&sphere_b));
    }

    #[test]
    fn sphere_sphere_intersection_just_shells_touching_test()  {
        let sphere_a = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 5.0);
        let mut sphere_b = Sphere::new(Vec3::new(0.0, 10.0, 0.0), 5.0);
        assert_eq!(false, sphere_a.does_intersect_with_sphere(&sphere_b));
        sphere_b.radius = 9.9;
        assert_eq!(true, sphere_a.does_intersect_with_sphere(&sphere_b));
    }
}