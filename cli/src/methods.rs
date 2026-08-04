use crate::models::{
    self, AddArgs, BranchArgs, CloneArgs, CompArgs, DesArgs, DiffArgs, DissArgs, EcArgs, ExpoArgs,
    FuseArgs, InitArgs, InsArgs, LogArgs, OpenArgs, RemArgs, RestArgs, SaveArgs, StatArgs,
    SwitchArgs, VeriArgs,
};
use std::path::PathBuf;

type Tokens<'a> = std::str::SplitWhitespace<'a>;

// ---------- Helper ---------- \\
fn parse_quoted(parts: &mut Tokens<'_>) -> Option<String> {
    let first = parts.next()?;

    let quote_char = match first.chars().next() {
        Some(c @ ('"' | '\'')) => c,
        _ => return Some(first.to_string()), // Not quoted, return as-is
    };

    // Remove the opening
    let mut reconstructed = first[1..].to_string();

    // If it already ends strip quotes and return
    if reconstructed.ends_with(quote_char) && reconstructed.len() >= 1 {
        reconstructed.pop();
        return Some(reconstructed);
    }

    // Keep consuming tokens until it is found the closing quote
    while let Some(next_token) = parts.next() {
        reconstructed.push(' ');

        if let Some(stripped) = next_token.strip_suffix(quote_char) {
            reconstructed.push_str(stripped);
            return Some(reconstructed);
        } else {
            reconstructed.push_str(next_token);
        }
    }

    // return what we accumulated, in case of not finding a quote: Crtical Behavior
    Some(reconstructed)
}

fn unexpected_argument(command: &str, parts: &mut Tokens<'_>) -> Result<(), String> {
    if let Some(arg) = parts.next() {
        Err(format!("{}: unexpected argument '{}'", command, arg))
    } else {
        Ok(())
    }
}

// ---------- Parsers ---------- \\
pub fn parse_init(mut parts: Tokens<'_>) -> Result<models::Commands, String> {
    let vault = parts.next().ok_or("Missing vault name")?;
    let loc = parts.next().ok_or("Missing Path")?;
    let primitive = parts
        .next()
        .ok_or("Missing primitive parameter")?
        .parse::<models::HashAlgo>()?;
    let method = parts
        .next()
        .ok_or("Missing method parameter")?
        .parse::<models::EncryptionMethod>()?;

    unexpected_argument("init", &mut parts)?;

    Ok(models::Commands::Init(InitArgs {
        vault: vault.to_string(),
        location: PathBuf::from(loc),
        primitive: primitive,
        method: method,
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
        all: all,
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
    let save = parts.next().map(String::from);

    unexpected_argument("status", &mut parts)?;

    Ok(models::Commands::Status(StatArgs { save: save }))
}

pub fn parse_save(mut parts: Tokens<'_>) -> Result<models::Commands, String> {
    let message = parse_quoted(&mut parts).ok_or("Missing message")?;
    let mut flags = Vec::new();

    while let Some(flag) = parts.next() {
        flags.push(flag.parse::<models::SaveFlags>()?);
    }

    Ok(models::Commands::Save(SaveArgs {
        message: message.into(),
        flags: if flags.is_empty() { None } else { Some(flags) },
    }))
}

pub fn parse_discard(mut parts: Tokens<'_>) -> Result<models::Commands, String> {
    let save = parts.next().ok_or("Missing save")?;

    unexpected_argument("discard", &mut parts)?;

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

    Ok(models::Commands::Compact(CompArgs {
        all: all,
        save: save,
    }))
}

pub fn parse_encrypt(mut parts: Tokens<'_>) -> Result<models::Commands, String> {
    let otype = parts
        .next()
        .ok_or("Missing method parameter")?
        .parse::<models::InternalObject>()
        .map_err(|e| e.to_string())?;

    let id = parts.next().ok_or("Missing save parameter")?.to_string();

    let plus_security = parts
        .next()
        .ok_or("Missing plus_security parameter")?
        .parse::<bool>()
        .map_err(|_| "plus_security must be 'true' or 'false'".to_string())?;

    let key = parts.next().map(String::from);

    let mut output = None;
    let mut apply_ps = Vec::new();

    while let Some(arg) = parts.next() {
        match arg {
            "-o" | "--output" => {
                output = parts.next().map(PathBuf::from);
            }
            file => apply_ps.push(PathBuf::from(file)),
        }
    }

    Ok(models::Commands::Encrypt(EcArgs {
        otype: otype,
        id: id,
        plus_security: plus_security,
        output: output,
        key: key,
        apply_ps: if apply_ps.is_empty() {
            None
        } else {
            Some(apply_ps)
        },
    }))
}

pub fn parse_log(mut parts: Tokens<'_>) -> Result<models::Commands, String> {
    let count = match parts.next() {
        Some(count_in_str) => Some(
            count_in_str
                .parse::<u16>()
                .map_err(|_| "Count must be a number between 0 and 65535".to_string())?,
        ),
        None => None,
    };

    let mut filters = Vec::new();

    while let Some(filter) = parts.next() {
        filters.push(filter.parse::<models::LogFilters>()?);
    }

    Ok(models::Commands::Log(LogArgs {
        count: count,
        filters: if filters.is_empty() {
            None
        } else {
            Some(filters)
        },
    }))
}

pub fn parse_open(mut parts: Tokens<'_>) -> Result<models::Commands, String> {
    let vault = parts.next().ok_or("Missing vault")?;

    unexpected_argument("open", &mut parts)?;

    Ok(models::Commands::Open(OpenArgs {
        vault: vault.into(),
    }))
}

pub fn parse_destroy(mut parts: Tokens<'_>) -> Result<models::Commands, String> {
    let vault = parts.next().ok_or("Missing vault")?;

    unexpected_argument("destroy", &mut parts)?;

    Ok(models::Commands::Destroy(DesArgs {
        vault: vault.into(),
    }))
}

pub fn parse_branch(mut parts: Tokens<'_>) -> Result<models::Commands, String> {
    let branch = parts.next().ok_or("Missing branch")?.into();
    let flag: Option<models::BranchFlags> = parts.next().map(|s| s.parse()).transpose()?;

    unexpected_argument("branch", &mut parts)?;

    Ok(models::Commands::Branch(BranchArgs {
        branch: branch,
        flag: flag,
    }))
}

pub fn parse_switch(mut parts: Tokens<'_>) -> Result<models::Commands, String> {
    let branch = parts.next().ok_or("Missing branch")?.into();

    unexpected_argument("switch", &mut parts)?;

    Ok(models::Commands::Switch(SwitchArgs { branch: branch }))
}

pub fn parse_fuse(mut parts: Tokens<'_>) -> Result<models::Commands, String> {
    let trunk_branch = parts.next().ok_or("Missing trunk branch")?.into();
    let feeder_branch = parts.next().ok_or("Missing feeder branch")?.into();
    let message = parse_quoted(&mut parts);
    let mut flags = Vec::new();

    while let Some(flag) = parts.next() {
        flags.push(flag.parse::<models::FuseFlags>()?);
    }

    Ok(models::Commands::Fuse(FuseArgs {
        branch1: trunk_branch,
        branch2: feeder_branch,
        message: message,
        flags: if flags.is_empty() { None } else { Some(flags) },
    }))
}

pub fn parse_clone(mut parts: Tokens<'_>) -> Result<models::Commands, String> {
    let dest_vault = parts.next().ok_or("Missing destination vault")?.to_string();
    let source_vault = parts.next().ok_or("Missing source vault")?.to_string();

    unexpected_argument("clone", &mut parts)?;

    Ok(models::Commands::Clone(CloneArgs {
        dest_vault: dest_vault,
        source_vault: source_vault,
    }))
}

pub fn parse_diff(mut parts: Tokens<'_>) -> Result<models::Commands, String> {
    let save1 = parts.next().map(String::from);
    let save2 = parts.next().map(String::from);
    let files: Vec<PathBuf> = parts.map(PathBuf::from).collect();

    Ok(models::Commands::Diff(DiffArgs {
        save1: save1,
        save2: save2,
        files: if files.is_empty() { None } else { Some(files) },
    }))
}

pub fn parse_restore(mut parts: Tokens<'_>) -> Result<models::Commands, String> {
    let save = parts.next().ok_or("Missing save")?.to_string();

    let overwrite = match parts.next() {
        Some(flag) => flag
            .parse::<bool>()
            .map_err(|_| "overwrite must be 'true' or 'false'".to_string())?,
        None => false,
    };

    let files: Vec<PathBuf> = parts.map(PathBuf::from).collect();

    Ok(models::Commands::Restore(RestArgs {
        save: save,
        overwrite: overwrite,
        files: if files.is_empty() { None } else { Some(files) },
    }))
}

pub fn parse_inspect(mut parts: Tokens<'_>) -> Result<models::Commands, String> {
    let save = parts.next().ok_or("Missing save")?.to_string();
    let mut flags = Vec::new();

    while let Some(flag) = parts.next() {
        flags.push(flag.parse::<models::InsFlags>()?);
    }

    Ok(models::Commands::Inspect(InsArgs {
        save: save,
        flags: if flags.is_empty() { None } else { Some(flags) },
    }))
}

pub fn parse_verify(mut parts: Tokens<'_>) -> Result<models::Commands, String> {
    let otype = parts
        .next()
        .ok_or("Missing object type parameter")?
        .parse::<models::InternalObject>()
        .map_err(|e| e.to_string())?;

    let id = parts.next().ok_or("Missing id parameter")?.to_string();

    unexpected_argument("verify", &mut parts)?;

    Ok(models::Commands::Verify(VeriArgs {
        otype: otype,
        id: id,
    }))
}

pub fn parse_export(mut parts: Tokens<'_>) -> Result<models::Commands, String> {
    let vault = parts.next().ok_or("Missing vault")?.to_string();
    let mut extreme_safety = false;
    let mut save = None;
    let mut destination = None;

    while let Some(arg) = parts.next() {
        match arg {
            "-xs" | "--extreme-safety" => extreme_safety = true,
            "-d" | "--destination" => {
                destination = parts.next().map(PathBuf::from);
            }
            other => {
                if save.is_none() {
                    save = Some(other.to_string());
                } else {
                    return Err(format!("export: unexpected argument '{}'", other));
                }
            }
        }
    }

    Ok(models::Commands::Export(ExpoArgs {
        vault: vault,
        extreme_safety: extreme_safety,
        save: save,
        destination: destination,
    }))
}
