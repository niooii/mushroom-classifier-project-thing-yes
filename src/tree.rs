pub struct Tree<T>
where T: PartialEq 
{
    data: Vec<Node<T>>
}

pub struct Node<T>
where T: PartialEq
{
    idx: usize,
    value: T,
    parent_idx: Option<usize>,
    children_idx: Vec<usize>
}