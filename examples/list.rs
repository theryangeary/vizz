use std::fs::File;
use vizz::Graph;
use vizz::Visualize;

#[derive(Visualize)]
enum List<T>
where
    T: Visualize,
{
    Nil,
    Cons { value: T, next: Box<List<T>> },
}

pub fn main() -> std::io::Result<()> {
    let my_list = List::Cons {
        value: String::from("first"),
        next: Box::from(List::Cons {
            value: String::from("second"),
            next: Box::from(List::Cons {
                value: String::from("third"),
                next: Box::from(List::Nil),
            }),
        }),
    };
    //
    // create file
    let mut dot_file = File::create("list.dot")?;

    // create graph
    Graph::new().add_node(&my_list).write_to(&mut dot_file)?;

    Ok(())
}
