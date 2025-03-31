use std::process::ChildStderr;

//? https://en.wikipedia.org/wiki/Quadtree

#[derive(Debug,Clone, Copy)]
struct Point{
    x:f32,
    y:f32
}

#[derive(Debug,Clone, Copy)]
struct DataPoint{
    x:f32,
    y:f32,
    i:usize
}

impl DataPoint {
    fn as_point(&self)->Point{
        Point { x: self.x, y: self.y }
    }
}

#[derive(Debug,Clone, Copy)]
struct AABB{
    center:Point,
    half_dim :f32,
}

#[derive(Clone,Copy,Debug)]
enum DiagonalDirection{
    UpRight,
    UpLeft,
    DownLeft,
    DownRight
}

impl AABB {
    fn contain_pt(&self, point:&Point)->bool{
        todo!()
    }

    fn intersection(&self, other : &Self)->Self{
        todo!()
    }

    fn diag_pos_from_center(&self,point:&Point)->DiagonalDirection{
        todo!()
    }
}

#[derive(Debug,Clone)]
struct Quadtree<T,const N:usize>{
    elem : Vec<T>,
    base_node :Node<N>
}

#[derive(Debug,Clone)]
struct Node<const N:usize>{
    boundary : AABB,
    data : NodeData<N>
}

#[derive(Debug,Clone)]
enum NodeData<const N:usize>{
    Child(NodeChildData<N>),
    Leaf(NodeLeafData<N>)
}

impl<const N:usize> Node<N>{
    fn subdivide(&mut self){
        if let NodeData::Leaf( leaf)=self.data{
            let child = leaf.subdivide_into_child_data(self.boundary);
            self.data = NodeData::Child(child)
        }

    }

    fn insert(&mut self,p:DataPoint)->Result<(),()>{

        if !self.boundary.contain_pt(&p.as_point()){
            return Err(());
        }

        if let NodeData::Child(childs) = &mut self.data{
            match self.boundary.diag_pos_from_center(&p.as_point()) {
                DiagonalDirection::UpLeft=>childs.up_left.insert(p)?,
                DiagonalDirection::UpRight=>childs.up_right.insert(p)?,
                DiagonalDirection::DownLeft=>childs.down_right.insert(p)?,
                DiagonalDirection::DownRight=>childs.down_right.insert(p)?,
            }
        }else if let NodeData::Leaf(leaf)= &mut self.data{
            if let Some(elem) =  leaf.data.get_mut(leaf.last_elem) {
                *elem = p;
                leaf.last_elem +=1;
            }else {
                self.subdivide();
                self.insert(p)?;
            }
        };

        Ok(())
    }

    fn querry_range(&self,range:AABB){
        todo!()
    }
}

#[derive(Debug,Clone)]
struct NodeChildData<const N:usize>{
    up_right:Box<Node<N>>,
    up_left: Box<Node<N>>,

    down_left : Box<Node<N>>,
    down_right: Box<Node<N>>
}

#[derive(Debug,Clone,Copy)]
struct NodeLeafData<const N:usize>{
    data : [DataPoint;N],
    last_elem : usize
}

impl<const N:usize> NodeLeafData<N>{
    fn subdivide_into_child_data(self,boundary:AABB)->NodeChildData<N>{
        todo!()
    }
}