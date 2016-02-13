pub mod test;

use test::*;



fn main() {
    let src = "MACHINE mch END";
    let res = parse_Component(src);
    println!("{:?}",res);
}
