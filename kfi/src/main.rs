use cli;

fn main() {
    let aresult = cli::parse("init blu ./Blu EE");
    let bresult = cli::parse("Save 'Lagoon azure' Kaurea --hard");
    println!("first atempt: {:?}\n second atempt: {:?}", aresult, bresult)
}
