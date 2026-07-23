use cli;

fn main() {
    let aresult = cli::parse("init Blu ./Blu");
    let bresult = cli::parse("Init blu ./blu");
    println!("first atempt: {}\n second atempt: {}" aresult, bresult)
}
