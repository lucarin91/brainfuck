use brainfuck::BrainFuck;

fn main() {
    let code = ">++++++++++++++++++++++++++++++++++++++++++++++++
    <++++++++++
    [
        >.+
        <-
    ]";
    let bf: BrainFuck = code.parse().unwrap();
    bf.run(); // out: 0123456789
}
