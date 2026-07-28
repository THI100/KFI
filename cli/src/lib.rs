pub mod methods;
pub mod models;

use crate::models::Commands;

// ---------- Functions ---------- \\

pub fn parse<T: AsRef<str>>(req: T) -> Result<Commands, String> {
    let mut parts = req.as_ref().split_whitespace();

    let command = parts
        .next()
        .map(|s| s.to_lowercase())
        .ok_or_else(|| "Empty command".to_string())?;

    match command.as_str() {
        "init" | "initialize" => methods::parse_init(parts),
        "add" => methods::parse_add(parts),
        "remove" | "rm" => methods::parse_remove(parts),
        "status" | "stats" => methods::parse_status(parts),
        "save" | "commit" => methods::parse_save(parts),
        "discard" => methods::parse_discard(parts),
        "compact" | "comp" => methods::parse_compact(parts),
        "encrypt" => methods::parse_encrypt(parts),
        "log" => methods::parse_log(parts),
        "open" => methods::parse_open(parts),
        "destroy" => methods::parse_destroy(parts),
        "branch" => methods::parse_branch(parts),
        "switch" => methods::parse_switch(parts),
        "fuse" | "merge" => methods::parse_fuse(parts),
        "clone" => methods::parse_clone(parts),
        "diff" | "differentiate" => methods::parse_diff(parts),
        "restore" | "res" => methods::parse_restore(parts),
        "inspect" => methods::parse_inspect(parts),
        "verify" => methods::parse_verify(parts),
        "export" => methods::parse_export(parts),
        _ => Err("Unknown command".into()),
    }
}
