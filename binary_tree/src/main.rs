use std::cmp::Ordering;


/*binary serach tree implementation*/

//The node of the tree with left subtree, right subtree and value
#[derive(Debug)]
struct Node<T:Ord>{
    left:SubTree<T>,
    right:SubTree<T>,
    value: T,
}


// subtree struct
#[derive(Debug)]
struct SubTree<T:Ord>(Option<Box<Node<T>>>);


//The tree struct tying everything together
#[derive(Debug)]
pub struct BTree<T:Ord>{
    root:SubTree<T>,
}

//the tree methods
impl<T:Ord> BTree<T> {
    fn new()->Self{
        Self { root: SubTree::new() }
    }
    fn insert(&mut self,value:T){
        self.root.insert(value);
    }
    fn has(&self, value: &T)-> bool{
       return self.root.has(value);
    }
    fn len(&self)->usize{
        return self.root.len();    
    }
    
}

// subtree methods
impl <T:Ord> SubTree<T> {
    fn new()->Self{
        Self(None)
        
    }
    fn insert(&mut self,value:T){
        match &mut self.0 {
            None =>self.0=Some(Box::new(Node::new(value))),
            Some(n)=>match value.cmp(&n.value) {
                   Ordering::Less=>n.left.insert(value),
                   Ordering::Equal=>n.value=value,
                   Ordering::Greater=>n.right.insert(value)    
            }
            
        }
    }

   fn has(&self, value:&T)->bool{
      match &self.0 {
        None=>false,
        Some(n)=>match value.cmp(&n.value) {
                  Ordering::Equal =>true,
                  Ordering::Greater =>n.right.has(value),
                  Ordering::Less=>n.left.has(value),
            
        },
        
    }
   }

   fn len(&self)->usize{
           match &self.0 {
            None=>0,
            Some(n)=>1 + n.left.len() + n.right.len(),
               
           }
   }    
}

//node methods
impl <T:Ord> Node<T> {
    fn new(value:T)->Self{
        Self { left: SubTree::new(), right: SubTree::new(), value }
    }
    
}

#[test]
fn test_insert(){
    let mut tree=BTree::new();
    tree.insert(1);
    tree.insert(23);
    tree.insert(30);
    let value=1;
    let value_two: i32=2;
    let value_three=30;
    let value_four=23;
    assert_eq!(tree.has(&value),true);
    assert_eq!(tree.has(&value_three),true);
    assert_eq!(tree.has(&value_four),true);
    assert_eq!(tree.has(&value_two),false);

}

#[test]
fn test_has(){
    let mut tree=BTree::new();
    tree.insert(20);
    tree.insert(10);
    let value=1;
    let value_two: i32=20;
    let value_three=10;
    assert_eq!(tree.has(&value_two),true);
    assert_eq!(tree.has(&value_three),true);
    assert_eq!(tree.has(&value),false);
}


#[test]
fn test_len(){
   let mut tree=BTree::new();
    tree.insert(1);
    assert_eq!(tree.len(),1);
    tree.insert(23);
    assert_eq!(tree.len(),2);
    tree.insert(30);
    assert_eq!(tree.len(),3);
    tree.insert(30);
    assert_eq!(tree.len(),3);
    tree.insert(1);
    assert_eq!(tree.len(),3);
    tree.insert(90);
    tree.insert(91);
    tree.insert(92);
    tree.insert(93);
    assert_eq!(tree.len(),7);
   
}

fn main() {

    let mut tree=BTree::new();
    tree.insert(1);
    
    println!()
}
