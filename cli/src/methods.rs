use crate::models::{
    self, AddArgs, ChangeArgs, CompArgs, DelArgs, DissArgs, EcArgs, InitArgs, LogArgs, RemArgs,
    RevArgs, SaveArgs, StatArgs,
};
use std::path::PathBuf;

type Tokens<'a> = std::str::SplitWhitespace<'a>;

pub fn parse_init(mut parts: Tokens<'_>) -> Result<models::Commands, String> {
    let vault = parts.next().ok_or("Missing vault name")?;
    let loc = parts.next().ok_or("Missing Path")?;

    Ok(models::Commands::Init(InitArgs {
        vault: vault.to_string(),
        location: PathBuf::from(loc),
    }))
}

pub fn parse_add(mut parts: Tokens<'_>) -> Result<models::Commands, String> {
    let mut all = false;
    let mut files = Vec::new();

    while let Some(arg) = parts.next() {
        match arg {
            "--all" | "-a" => all = true,
            file => files.push(PathBuf::from(file)),
        }
    }

    Ok(models::Commands::Add(AddArgs {
        all,
        files: if files.is_empty() { None } else { Some(files) },
    }))
}

pub fn parse_remove(mut parts: Tokens<'_>) -> Result<models::Commands, String> {
    let mut files = Vec::new();

    while let Some(file) = parts.next() {
        files.push(PathBuf::from(file));
    }

    Ok(models::Commands::Remove(RemArgs { files: files }))
}

pub fn parse_status(mut parts: Tokens<'_>) -> Result<models::Commands, String> {
    let save = parts.next().ok_or("Missing save")?;

    Ok(models::Commands::Status(StatArgs { save: save.into() }))
}

pub fn parse_save(mut parts: Tokens<'_>) -> Result<models::Commands, String> {
    let message = parts.next().ok_or("Missing message")?;
    let primitive = parts
        .next()
        .ok_or("Missing primitive parameter")?
        .parse::<models::HashAlgo>()?;
    let mut flags = Vec::new();

    while let Some(flag) = parts.next() {
        flags.push(flag.into());
    }

    Ok(models::Commands::Save(SaveArgs {
        message: message.into(),
        primitive: primitive,
        flags: if flags.is_empty() { None } else { Some(flags) },
    }))
}

pub fn parse_discard(mut parts: Tokens<'_>) -> Result<models::Commands, String> {
    let save = parts.next().ok_or("Missing save")?;

    Ok(models::Commands::Discard(DissArgs { save: save.into() }))
}

pub fn parse_compact(mut parts: Tokens<'_>) -> Result<models::Commands, String> {
    let mut all = false;
    let mut save = None;

    if let Some(arg) = parts.next() {
        if arg == "-a" || arg == "--all" || arg == "." {
            all = true;
            save = parts.next().map(String::from);
        } else {
            save = Some(arg.to_string());
        }
    }

    Ok(models::Commands::Compact(CompArgs { all, save }))
}

pub fn parse_encrypt(mut parts: Tokens<'_>) -> Result<models::Commands, String> {
    let save = parts.next().ok_or("Missing save parameter")?.to_string();

    let method = parts
        .next()
        .ok_or("Missing method parameter")?
        .parse::<models::EncryptionMethod>()
        .map_err(|e| e.to_string())?;

    let plus_security = parts
        .next()
        .ok_or("Missing plus_security parameter")?
        .parse::<bool>()
        .map_err(|_| "plus_security must be 'true' or 'false'".to_string())?;

    let key = parts.next().map(String::from);

    let apply_ps: Vec<PathBuf> = parts.map(PathBuf::from).collect();

    Ok(models::Commands::Encrypt(EcArgs {
        save,
        method,
        plus_security,
        key,
        apply_ps: if apply_ps.is_empty() {
            None
        } else {
            Some(apply_ps)
        },
    }))
}

pub fn parse_log(mut parts: Tokens<'_>) -> Result<models::Commands, String> {
    let count_in_str = parts.next().ok_or("Missing count")?;

    let count = count_in_str.parse::<u8>().unwrap();

    let filters: Vec<String> = parts.map(String::from).collect();

    Ok(models::Commands::Log(LogArgs {
        count: count,
        filters: if filters.is_empty() {
            None
        } else {
            Some(filters)
        },
    }))
}

pub fn parse_revert(mut parts: Tokens<'_>) -> Result<models::Commands, String> {
    let save = parts.next().ok_or("Missing save")?;
    let file = parts.next().map(PathBuf::from);

    Ok(models::Commands::Revert(RevArgs {
        save: save.into(),
        file: file,
    }))
}

pub fn parse_change(mut parts: Tokens<'_>) -> Result<models::Commands, String> {
    let vault = parts.next().ok_or("Missing vault")?;

    Ok(models::Commands::Change(ChangeArgs {
        vault: vault.into(),
    }))
}

pub fn parse_delete(mut parts: Tokens<'_>) -> Result<models::Commands, String> {
    let vault = parts.next().ok_or("Missing vault")?;

    Ok(models::Commands::Delete(DelArgs {
        vault: vault.into(),
    }))
}
