use vizz::Visualize;

pub fn main() {
    let my_int: u8 = 128;
    let my_int_dot = (&my_int).render_node();
    println!("{}", my_int_dot);
}
