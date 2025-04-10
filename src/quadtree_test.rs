#[cfg(test)]
mod tests {
    use crate::quadtree::{AABB, As2dPoint, Point, Quadtree};

    #[test]
    fn test_aabb_contain_point1() {
        let aabb = AABB::new((5.0, 5.0), 3.0);
        let inside_point = Point { x: 6.0, y: 6.0 };
        let outside_point = Point { x: 9.0, y: 9.0 };

        assert!(aabb.contain_pt(&inside_point));
        assert!(!aabb.contain_pt(&outside_point));
    }

    #[test]
    fn test_aabb_intersect() {
        let aabb1 = AABB::new((5.0, 5.0), 3.0);
        let aabb2 = AABB::new((6.0, 6.0), 3.0);
        let aabb3 = AABB::new((10.0, 10.0), 2.0);

        assert!(aabb1.intersect(&aabb2));
        assert!(!aabb1.intersect(&aabb3));
    }

    #[test]
    fn test_aabb_subdivide() {
        let aabb = AABB::new((5.0, 5.0), 4.0);
        let quadrants = aabb.subdivide();

        assert_eq!(quadrants.len(), 4);
        assert_eq!(quadrants[0].half_dim, 2.0);
    }

    #[derive(Debug, Clone)]
    struct TestPoint {
        x: f32,
        y: f32,
    }

    impl As2dPoint for TestPoint {
        fn x(&self) -> f32 {
            self.x
        }
        fn y(&self) -> f32 {
            self.y
        }
    }

    #[test]
    fn test_quadtree_insert() {
        let boundary = AABB::new((5.0, 5.0), 5.0);
        let mut quadtree: Quadtree<TestPoint, 4> = Quadtree::empty(boundary);

        let point = TestPoint { x: 6.0, y: 6.0 };
        assert!(quadtree.insert(point).is_ok());
    }

    #[test]
    fn test_quadtree_query_range() {
        let boundary = AABB::new((5.0, 5.0), 5.0);
        let mut quadtree: Quadtree<TestPoint, 4> = Quadtree::empty(boundary);

        let point = TestPoint { x: 4.0, y: 4.0 };
        quadtree.insert(point).unwrap();

        let query_range = AABB::new((4.0, 4.0), 1.0);
        let result = quadtree.query_range(query_range);

        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_rebuild() {
        let boundary = AABB::new((0., 0.), 100.);
        let mut qtree: Quadtree<TestPoint, 3> = Quadtree::empty(boundary);

        let points = vec![
            TestPoint { x: 4.0, y: 4.0 },
            TestPoint { x: 8.0, y: 4.0 },
            TestPoint { x: 10.0, y: 5.0 },
            TestPoint { x: 25.0, y: -1.0 },
        ];
        for p in points {
            qtree.insert(p).unwrap();
        }
    }

    #[test]
    #[ignore]
    fn test_lot_of_insert() {
        // on my pc -> 11.39s
        const POINT_NB: u32 = 10_000_000;
        let half_dim = POINT_NB as f32;

        let boundary = AABB::new((0., 0.), half_dim + 500.);
        let mut qtree: Quadtree<TestPoint, 3> = Quadtree::empty(boundary);
        for i in 0..POINT_NB {
            let i_f = half_dim - i as f32 / 2.;
            let p = TestPoint {
                x: i_f,
                y: f32::sin(i as f32) * i_f,
            };
            qtree.insert(p).unwrap();
        }
    }
}
