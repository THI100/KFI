use cli;

fn main() {
    let aresult = cli::parse("init blu ./Blu EE");
    let bresult = cli::parse("Init Blu ./blu");
    println!("first atempt: {:?}\n second atempt: {:?}", aresult, bresult)
}
