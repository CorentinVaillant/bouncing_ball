#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: f32,
    y: f32,
}

impl From<(f32, f32)> for Point {
    fn from((x, y): (f32, f32)) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
struct IndexPoint {
    x: f32,
    y: f32,
    i: usize,
}

impl IndexPoint {
    fn as_point(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AABB {
    center: Point,
    half_dim: f32,
}

#[derive(Clone, Copy, Debug)]
enum DiagonalDirection {
    UpRight,
    UpLeft,
    DownLeft,
    DownRight,
}

impl AABB {
    fn contain_pt(&self, point: &Point) -> bool {
        (point.x - self.center.x).abs() < self.half_dim
            && (point.y - self.center.y).abs() < self.half_dim
    }

    fn intersect(&self, other: &Self) -> bool {
        let self_max: Point = (self.center.x + self.half_dim, self.center.y + self.half_dim).into();
        let self_min: Point = (self.center.x - self.half_dim, self.center.y - self.half_dim).into();
        let other_max: Point = (
            other.center.x + other.half_dim,
            other.center.y + other.half_dim,
        )
            .into();
        let other_min: Point = (
            other.center.x - other.half_dim,
            other.center.y - other.half_dim,
        )
            .into();

        (self_min.x <= other_max.x && self_max.x >= other_min.x)
            && (self_min.y <= other_max.y && self_max.y >= other_min.y)
    }

    fn diag_pos_from_center(&self, point: &Point) -> DiagonalDirection {
        match (point.x >= self.center.x, point.y >= self.center.y) {
            (false, false) => DiagonalDirection::DownLeft,
            (false, true) => DiagonalDirection::UpLeft,
            (true, false) => DiagonalDirection::DownRight,
            (true, true) => DiagonalDirection::UpRight,
        }
    }

    fn subdivide(self) -> [Self; 4] {
        let quart_dim = self.half_dim / 2.;

        [
            AABB {
                center: (self.center.x - quart_dim, self.center.y + quart_dim).into(),
                half_dim: quart_dim,
            },
            AABB {
                center: (self.center.x + quart_dim, self.center.y + quart_dim).into(),
                half_dim: quart_dim,
            },
            AABB {
                center: (self.center.x + quart_dim, self.center.y - quart_dim).into(),
                half_dim: quart_dim,
            },
            AABB {
                center: (self.center.x - quart_dim, self.center.y - quart_dim).into(),
                half_dim: quart_dim,
            },
        ]
    }
}

pub trait As2dPoint {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
}
#[derive(Debug, Clone)]
pub struct Quadtree<T: As2dPoint, const N: usize> {
    vec: Vec<T>,
    base_node: Node<N>,
}

type OutOfBoundary = ();
impl<T: As2dPoint, const N: usize> Quadtree<T, N> {

    fn insert(&mut self, elem: T) -> Result<(), OutOfBoundary> {
        let i = self.vec.len();
        let i_p = IndexPoint {
            x: elem.x(),
            y: elem.y(),
            i,
        };

        self.vec.push(elem);

        self.base_node.insert(i_p)
    }

    fn query_range(&self, range: AABB) -> Vec<&T> {
        let mut result = vec![];
        for i_p in self.base_node.query_range(range) {
            result.push(&self.vec[i_p.i]);
        }

        result
    }

    //todo
    // fn query_range_mut(&mut self, range: AABB)->Vec<&mut T>{
    //     let mut result = vec![];
    //     for i_point in self.base_node.query_range(range) {
    //         result.push( &mut self.vec[i_point.i]);
    //     }

    //     result
    // }
}

#[derive(Debug, Clone)]
struct Node<const N: usize> {
    boundary: AABB,
    data: NodeData<N>,
}

impl<const N: usize> Node<N> {
    fn subdivide(&mut self) {
        if let NodeData::Leaf(leaf) = self.data {
            let child = leaf.subdivide_into_child_data(self.boundary);
            self.data = NodeData::Child(child)
        }
    }

    fn insert(&mut self, p: IndexPoint) -> Result<(), ()> {
        if !self.boundary.contain_pt(&p.as_point()) {
            return Err(());
        }

        if let NodeData::Child(childs) = &mut self.data {
            match self.boundary.diag_pos_from_center(&p.as_point()) {
                DiagonalDirection::UpLeft => childs.up_left.insert(p)?,
                DiagonalDirection::UpRight => childs.up_right.insert(p)?,
                DiagonalDirection::DownLeft => childs.down_left.insert(p)?,
                DiagonalDirection::DownRight => childs.down_right.insert(p)?,
            }
        } else if let NodeData::Leaf(leaf) = &mut self.data {
            if let Some(elem) = leaf.points.get_mut(leaf.next_i) {
                debug_assert!(leaf.next_i < N, "NodeLeafData overflow: exceeded capacity {}", N);
                *elem = Some(p);
                leaf.next_i += 1;
            } else {
                self.subdivide();
                return self.insert(p);
            }
        };

        Ok(())
    }

    fn query_range(&self, range: AABB) -> Vec<IndexPoint> {
        let mut result = Vec::new();

        if !self.boundary.intersect(&range) {
            result
        } else {
            match &self.data {
                NodeData::Child(child) => {
                    result.append(&mut child.up_right.query_range(range));
                    result.append(&mut child.up_left.query_range(range));
                    result.append(&mut child.down_left.query_range(range));
                    result.append(&mut child.down_right.query_range(range));
                }
                NodeData::Leaf(leaf) => {
                    for i in &leaf.points[0..leaf.next_i] {
                        if let Some(i) = i {
                            result.push(*i);
                        }
                    }
                }
            }
            result
        }
    }
}

#[derive(Debug, Clone)]
enum NodeData<const N: usize> {
    Child(NodeChildData<N>),
    Leaf(NodeLeafData<N>),
}

impl<const N: usize> NodeData<N> {
    fn new_leaf(points: Vec<IndexPoint>) -> Self {
        let mut points_arr = [None; N];
        let mut next_i = N;
        for i in 0..N {
            if let Some(pt) = points.get(i) {
                points_arr[i] = Some(*pt);
            } else {
                next_i = i;
                break;
            }
        }

        Self::Leaf(NodeLeafData {
            points: points_arr,
            next_i,
        })
    }
}

#[derive(Debug, Clone)]
struct NodeChildData<const N: usize> {
    up_right: Box<Node<N>>,
    up_left: Box<Node<N>>,

    down_left: Box<Node<N>>,
    down_right: Box<Node<N>>,
}

#[derive(Debug, Clone, Copy)]
struct NodeLeafData<const N: usize> {
    points: [Option<IndexPoint>; N],
    next_i: usize,
}

impl<const N: usize> NodeLeafData<N> {
    fn subdivide_into_child_data(self, boundary: AABB) -> NodeChildData<N> {
        let [ur, ul, dl, dr] = boundary.subdivide();
        let [mut ur_p, mut ul_p, mut dl_p, mut dr_p] = [const { vec![] }; 4];
        for p in self.points {
            if let Some(p) = p {
                match boundary.diag_pos_from_center(&p.as_point()) {
                    DiagonalDirection::UpLeft => ul_p.push(p),
                    DiagonalDirection::UpRight => ur_p.push(p),
                    DiagonalDirection::DownRight => dr_p.push(p),
                    DiagonalDirection::DownLeft => dl_p.push(p),
                }
            }
        }

        let up_right = Box::new(Node {
            boundary: ur,
            data: NodeData::new_leaf(ur_p),
        });
        let up_left = Box::new(Node {
            boundary: ul,
            data: NodeData::new_leaf(ul_p),
        });
        let down_left = Box::new(Node {
            boundary: dl,
            data: NodeData::new_leaf(dl_p),
        });
        let down_right = Box::new(Node {
            boundary: dr,
            data: NodeData::new_leaf(dr_p),
        });

        NodeChildData {
            up_right,
            up_left,
            down_left,
            down_right,
        }
    }
}
