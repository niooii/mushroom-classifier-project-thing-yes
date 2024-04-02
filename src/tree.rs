pub struct Tree<T>
where T: PartialEq 
{
    data: Vec<Node<T>>
}

impl<T> Tree<T> 
where T: PartialEq
{
    pub fn new(root_val: T) -> Self {
        Self {
            data: vec![Node::new(0, root_val)]
        }
    }

    pub fn root(&self) -> &Node<T> {
        &self.data[0]
    }

    pub fn root_mut(&mut self) -> &mut Node<T> {
        &mut self.data[0]
    }
    
    pub fn get_children_of(&self, node: &Node<T>) -> Vec<&Node<T>> {
        let mut vec = Vec::new();

        for node_idx in &node.children_idx {
            vec.push(&self.data[*node_idx]);
        }

        vec
    }
    
    pub fn get_children_of_mut(&mut self, node: &Node<T>) -> Vec<&mut Node<T>> {
        self.data.iter_mut()
            .enumerate()
            .filter(|(i, _v)| node.children_idx.contains(i))
            .map(|(_i, v)| v).collect()
    }
    
    pub fn get_parent_of(&self, node: &Node<T>) -> Option<&Node<T>> {
        node.parent_idx.and_then(|parent_idx| self.data.get(parent_idx))
    }
    
    pub fn get_parent_of_mut(&mut self, node: &Node<T>) -> Option<&mut Node<T>> {
        node.parent_idx.and_then(|parent_idx| self.data.get_mut(parent_idx))
    }

    // - _ -
    pub fn make_child(&mut self, parent_node: &Node<T>, node_val: T) {
        let free_idx = self.data.len();
        self.data[parent_node.idx].children_idx.push(free_idx);
        self.data.push(Node::new(free_idx, node_val));
        self.data[free_idx].parent_idx = Some(parent_node.idx);
    }
}

pub struct Node<T>
where T: PartialEq
{
    idx: usize,
    pub value: T,
    parent_idx: Option<usize>,
    children_idx: Vec<usize>
}

impl<T> PartialEq for Node<T> 
where T: PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        self.idx == other.idx
    }
}

impl<T> Node<T>
where T: PartialEq 
{
    fn new(idx: usize, value: T) -> Self {
        Self {
            idx,
            value,
            parent_idx: None,
            children_idx: Vec::new()
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.children_idx.len() == 0
    }
}