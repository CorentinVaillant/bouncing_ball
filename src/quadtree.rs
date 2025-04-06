#![allow(dead_code)]

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    //false if either x or y are NaN or Infinite
    fn is_valid(&self) -> bool {
        !(self.x.is_nan() || self.y.is_nan() || self.x.is_infinite() || self.y.is_infinite())
    }
}

impl From<(f32, f32)> for Point {
    fn from((x, y): (f32, f32)) -> Self {
        Self { x, y }
    }
}

impl From<Point> for (f32, f32) {
    fn from(value: Point) -> Self {
        (value.x, value.y)
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

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy)]
pub struct AABB {
    pub center: Point,
    pub half_dim: f32,
}

#[derive(Clone, Copy, Debug)]
enum DiagonalDirection {
    UpRight,
    UpLeft,
    DownLeft,
    DownRight,
}

impl AABB {
    pub fn new(center: (f32, f32), half_width: f32) -> Self {
        debug_assert!(half_width > 0., "half width should always be > 0.");
        Self {
            center: center.into(),
            half_dim: half_width,
        }
    }

    pub fn contain_pt(&self, point: &Point) -> bool {
        let dx = (point.x - self.center.x).abs();
        let dy = (point.y - self.center.y).abs();
        dx <= self.half_dim && dy <= self.half_dim
    }

    pub fn intersect(&self, other: &Self) -> bool {
        let dx = (self.center.x - other.center.x).abs();
        let dy = (self.center.y - other.center.y).abs();
        dx < (self.half_dim + other.half_dim) && dy < (self.half_dim + other.half_dim)
    }

    pub fn contain_aabb(&self, other: &Self) -> bool {
        let self_min_x = self.center.x - self.half_dim;
        let self_max_x = self.center.x + self.half_dim;
        let self_min_y = self.center.y - self.half_dim;
        let self_max_y = self.center.y + self.half_dim;
    
        let other_min_x = other.center.x - other.half_dim;
        let other_max_x = other.center.x + other.half_dim;
        let other_min_y = other.center.y - other.half_dim;
        let other_max_y = other.center.y + other.half_dim;
    
        self_min_x <= other_min_x
            && self_max_x >= other_max_x
            && self_min_y <= other_min_y
            && self_max_y >= other_max_y
    }
    

    fn diag_pos_from_center(&self, point: &Point) -> DiagonalDirection {
        match (point.x > self.center.x, point.y > self.center.y) {
            (false, false) => DiagonalDirection::DownLeft,
            (false, true) => DiagonalDirection::UpLeft,
            (true, false) => DiagonalDirection::DownRight,
            (true, true) => DiagonalDirection::UpRight,
        }
    }

    pub fn subdivide(self) -> [Self; 4] {
        let quart_dim = self.half_dim / 2.;
        let offsets = [(-1., 1.), (1., 1.), (1., -1.), (-1., -1.)];

        offsets.map(|(dx, dy)| Self {
            center: (
                self.center.x + dx * quart_dim,
                self.center.y + dy * quart_dim,
            )
                .into(),
            half_dim: quart_dim,
        })
    }
}

pub trait As2dPoint {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn set_pos(&mut self, pos: (f32, f32));

    fn as_point(&self) -> Point {
        (self.x(), self.y()).into()
    }
}

#[derive(Debug, Clone)]
pub struct Quadtree<T: As2dPoint, const N: usize> {
    vec: Vec<T>,
    base_node: Node<N>,
}

#[derive(Debug, Clone, Copy)]
pub enum QuadtreeError {
    OutOfBoundary(AABB, (f32, f32)),
    InvalidCoord((f32, f32)),
}

impl<T: As2dPoint, const N: usize> Quadtree<T, N> {
    pub fn empty(boundary: AABB) -> Self {
        debug_assert!(N > 0, "The size should be a least 1");

        Self {
            vec: vec![],
            base_node: Node::empty(boundary),
        }
    }

    pub fn new(boundary: AABB, vec: Vec<T>) -> Self {
        debug_assert!(N > 0, "The size should be a least 1");

        let mut result = Self {
            vec,
            base_node: Node::empty(boundary),
        };
        result.rebuild_fit();
        result
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn depth(&self) -> usize {
        self.base_node.depth()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.vec.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.vec.iter_mut()
    }

    pub fn insert(&mut self, elem: T) -> Result<(), QuadtreeError> {
        let i = self.vec.len();
        let i_p = IndexPoint {
            x: elem.x(),
            y: elem.y(),
            i,
        };

        if !self.base_node.boundary.contain_pt(&i_p.as_point()) {
            return Err(QuadtreeError::OutOfBoundary(
                self.base_node.boundary,
                (i_p.x, i_p.y),
            ));
        }

        self.vec.push(elem);

        self.base_node.insert(i_p)
          .expect("something went wrong in QuadTree::insert: could not insert the value, even if it is in the Tree boundary\n\t=>");

        Ok(())
    }

    pub fn insert_fit(&mut self, elem: T) {
        let i = self.vec.len();
        let i_p = IndexPoint {
            x: elem.x(),
            y: elem.y(),
            i,
        };

        self.vec.push(elem);

        if self.base_node.insert(i_p).is_err() {
            self.rebuild_fit();
        }
    }

    pub fn query_range(&self, range: AABB) -> Vec<&T> {
        let mut result = vec![];
        for i_p in self.base_node.query_range(range) {
            result.push(&self.vec[i_p.i]);
        }

        result
    }

    pub fn map_query_range(&mut self, range: AABB, map: impl Fn(&mut T)) {
        for i_point in self.base_node.query_range(range) {
            map(&mut self.vec[i_point.i]);
        }
    }

    pub fn map_with_elem_in_range(
        &mut self,
        range_mapping: impl Fn(&T) -> AABB,
        map: impl Fn(&mut T, &mut T),
    ) {
        for i in 0..self.vec.len() {
            let range = self.base_node.query_range(range_mapping(&self.vec[i]));

            for p in range {
                match p.i.cmp(&i) {
                    std::cmp::Ordering::Greater => {
                        let (split_i, split_p) = self.vec.split_at_mut(p.i);
                        map(&mut split_i[i], &mut split_p[0]);
                        // println!("2. \t=> searching next")
                    }
                    std::cmp::Ordering::Less => {
                        let (split_p, split_i) = self.vec.split_at_mut(i);
                        map(&mut split_p[p.i], &mut split_i[0]);
                        // println!("2. \t=> searching next")
                    }
                    _ => (),
                };
            }
        }
    }

    //Horible name
    ///For each point in the quadtree :
    /// 1. first_map(point)  
    /// 2. for each other in range_mapping(point) :  
    /// ->  2.1 map_with_other(point,other)  
    /// 3. last_map(point)  
    pub fn map_the_map_with_elem_in_range_then_map(
        &mut self,
        first_map: impl Fn(&mut T),
        range_mapping: impl Fn(&T) -> AABB,
        map_with_other: impl Fn(&mut T, &mut T),
        last_map: impl Fn(&mut T),
    ) {

        let mut new_base_node = Node::empty(self.base_node.boundary);
        let mut failed_to_insert = false;

        for i in 0..self.vec.len() {
            let range = self.base_node.query_range(range_mapping(&self.vec[i]));

            first_map(&mut self.vec[i]);

            for p in range {
                match p.i.cmp(&i) {
                    std::cmp::Ordering::Greater => {
                        let (split_i, split_p) = self.vec.split_at_mut(p.i);
                        map_with_other(&mut split_i[i], &mut split_p[0]);
                        // println!("2. \t=> searching next")
                    }
                    std::cmp::Ordering::Less => {
                        let (split_p, split_i) = self.vec.split_at_mut(i);
                        map_with_other(&mut split_p[p.i], &mut split_i[0]);
                        // println!("2. \t=> searching next")
                    }
                    _ => (),
                };
            }
            last_map(&mut self.vec[i]);

            if !failed_to_insert{
                let new_i_pt = IndexPoint{x:self.vec[i].x(), y:self.vec[i].y(), i};
                failed_to_insert = new_base_node.insert(new_i_pt).is_err();
            }
        }
        if !failed_to_insert{
            self.base_node = new_base_node;
        }else {
            println!("rebuild the entiere tree");
            self.rebuild_fit();
        }
    }

    const MIN_SIZE: f32 = 0.01;

    pub fn rebuild_fit(&mut self) {
        if !self
            .vec
            .iter()
            .all(|p| self.base_node.boundary.contain_pt(&p.as_point()))
        {
            let (min_x, max_x, min_y, max_y) = self.vec.iter().fold(
                (
                    f32::INFINITY,
                    f32::NEG_INFINITY,
                    f32::INFINITY,
                    f32::NEG_INFINITY,
                ),
                |(min_x, max_x, min_y, max_y), elem| {
                    (
                        min_x.min(elem.x()) - 1.,
                        max_x.max(elem.x()) + 1.,
                        min_y.min(elem.y()) - 1.,
                        max_y.max(elem.y()) + 1.,
                    )
                },
            );
    
            let width = max_x - min_x;
            let height = max_y - min_y;
            let new_half_width = (width.max(height) / 2.).max(Self::MIN_SIZE);
            let new_center = ((min_x + max_x) / 2., (min_y + max_y) / 2.).into();
    
            self.base_node = Node::empty(AABB::new(new_center, new_half_width));
        } else {
            self.base_node = Node::empty(self.base_node.boundary);
        }
    
        for (i, elem) in self.vec.iter().enumerate() {
            let elem_pt = IndexPoint {
                x: elem.x(),
                y: elem.y(),
                i,
            };
            if let Err(e) = self.base_node.insert(elem_pt) {
                match e {
                    QuadtreeError::OutOfBoundary(_, _) => panic!(
                        "QuadTree::rebuild went wrong : All points should fit after resize\n\t=>{e:?}"
                    ),
                    QuadtreeError::InvalidCoord(_) => {
                        panic!(
                            "QuadTree::rebuild went wrong : elem: {i} does not have valid coordinate\n\t=>{e:?}"
                        )
                    }
                }
            }
        }
    }
    
    pub fn rebuild(&mut self)->Result<(),QuadtreeError>{
        let mut new_node = Node::empty(self.base_node.boundary);

        for (i, elem) in self.vec.iter().enumerate(){
            let i_pt = IndexPoint{
                x:elem.x(),
                y:elem.y(),
                i
            };

            new_node.insert(i_pt)?;
        }
        self.base_node = new_node;
        Ok(())

    }

    pub fn change_bounds(&mut self, new_bound: AABB) -> Result<(), QuadtreeError> {
        let mut new_node = Node::empty(new_bound);
        for (i, elem) in self.vec.iter().enumerate() {
            let elem_pt = IndexPoint {
                x: elem.x(),
                y: elem.y(),
                i,
            };

            new_node.insert(elem_pt)?;
        }

        self.base_node = new_node;

        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Node<const N: usize> {
    boundary: AABB,
    data: NodeData<N>,
}

impl<const N: usize> Node<N> {
    fn empty(boundary: AABB) -> Self {
        Self {
            boundary,
            data: NodeData::Leaf(NodeLeafData {
                points: [None; N],
                next_i: 0,
            }),
        }
    }

    fn subdivide(&mut self) {
        if let NodeData::Leaf(leaf) = self.data {
            let child = leaf.subdivide_into_child_data(self.boundary);
            self.data = NodeData::Child(child)
        }
    }

    fn insert(&mut self, p: IndexPoint) -> Result<(), QuadtreeError> {
        if !p.as_point().is_valid() {
            return Err(QuadtreeError::InvalidCoord(p.as_point().into()));
        }

        if !self.boundary.contain_pt(&p.as_point()) {
            return Err(QuadtreeError::OutOfBoundary(self.boundary, (p.x, p.y)));
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
                debug_assert!(
                    leaf.next_i < N,
                    "NodeLeafData overflow: exceeded capacity {}",
                    N
                );
                *elem = Some(p);
                leaf.next_i += 1;
            } else {
                self.subdivide();
                self.insert(p)
                    .expect("something went wrong in quadtree.rs : Node::insert: could not insert the value, even if it is in the Node boundary\n\t=>");
            }
        };

        Ok(())
    }

    fn depth(&self) -> usize {
        1 + match &self.data {
            NodeData::Child(node_child_data) => node_child_data
                .down_left
                .depth()
                .max(node_child_data.down_right.depth())
                .max(
                    node_child_data
                        .up_left
                        .depth()
                        .max(node_child_data.up_right.depth()),
                ),
            _ => 0,
        }
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
                    for i in leaf.points[0..leaf.next_i].iter().flatten() {
                        result.push(*i);
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

        for (i, p) in points_arr.iter_mut().enumerate().take(N) {
            if let Some(pt) = points.get(i) {
                *p = Some(*pt);
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
        let [ul, ur, dr, dl] = boundary.subdivide();
        let [mut ur_p, mut ul_p, mut dl_p, mut dr_p] = [const { vec![] }; 4];
        for p in self.points.iter().flatten() {
            {
                match boundary.diag_pos_from_center(&p.as_point()) {
                    DiagonalDirection::UpLeft => ul_p.push(*p),
                    DiagonalDirection::UpRight => ur_p.push(*p),
                    DiagonalDirection::DownRight => dr_p.push(*p),
                    DiagonalDirection::DownLeft => dl_p.push(*p),
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
