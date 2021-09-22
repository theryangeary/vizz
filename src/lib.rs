use std::io::Write;

trait Dot<W>
where
    W: Write,
{
    fn build(&self, &mut graph: DotGraph);
    fn render_to(&self, &mut writable: W) -> Result<(), ()>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
