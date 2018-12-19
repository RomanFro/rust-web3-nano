mod eth;

fn main() {
    let res = eth::get_block();

    println!("{:?}", res);
}
