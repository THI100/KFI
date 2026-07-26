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
        "revert" => methods::parse_revert(parts),
        "change" | "switch" => methods::parse_change(parts),
        "delete" | "del" => methods::parse_delete(parts),
        "branch" => methods::parse_branch(parts),
        "checkout" => methods::parse_checkout(parts),
        "fuse" | "merge" => methods::parse_fuse(parts),
        _ => Err("Unknown command".into()),
    }
}
