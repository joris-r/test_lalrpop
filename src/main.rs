pub mod test;

use test::*;



fn main() {
    let src = "IMPLEMENTATION ref REFINES mch END";
    let res = parse_Component(src);
    println!("{:?}",res);
}
