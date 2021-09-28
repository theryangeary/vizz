use std::fs::File;
use vizz::Graph;
use vizz::Visualize;

#[derive(Visualize)]
struct Tree<T>
where
    T: Visualize,
{
    left: Option<Box<Tree<T>>>,
    value: T,
    right: Option<Box<Tree<T>>>,
}

pub fn main() -> std::io::Result<()> {
    let a = Tree {
        left: None,
        value: 7,
        right: None,
    };
    let b = Tree {
        left: None,
        value: 5,
        right: None,
    };
    let c = Tree {
        left: Some(Box::from(a)),
        value: 6,
        right: Some(Box::from(b)),
    };
    let d = Tree {
        left: None,
        value: 1,
        right: None,
    };
    let e = Tree {
        left: None,
        value: 3,
        right: None,
    };
    let f = Tree {
        left: Some(Box::from(d)),
        value: 2,
        right: Some(Box::from(e)),
    };
    let g = Tree {
        left: Some(Box::from(c)),
        value: 4,
        right: Some(Box::from(f)),
    };

    // create file
    let mut dot_file = File::create("tree.dot")?;

    // create graph
    Graph::from(&g).write_to(&mut dot_file)?;

    Ok(())
}
