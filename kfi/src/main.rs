use std::env;
use std::io::{self, Write};

use cli;
use runner;

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_commands() {
        let inputs = [
            ("init blu ./Blu", true),
            ("add -a file3 file2", true),
            ("remove file3 file2", true),
            ("Status #faodpvs3053254135..", true),
            ("save 'laga vaken' Blake3 --sign", true),
            ("discard #faodpvs3053254135..", true),
            ("comp -a", true),
            ("Encrypt save #faodpvs3053254135.. aes false", true),
            ("log 2", true),
            ("Open fanf", true),
            ("Destroy mac", true),
            ("branch a -N", true),
            ("switch b", true),
            ("fuse a b -i", true),
            ("clone new_vault old_vault", true),
            ("diff #faodpvs3053254135.. #bcfe093a1153ac.. file1", true),
            ("restore #faodpvs3053254135.. true file1 file2", true),
            ("inspect #faodpvs3053254135.. -det", true),
            ("verify vault blu", true),
            ("export blu -xs #faodpvs3053254135.. -d ./backup", true),
            ("monkeysssssss", false),
        ];
        for (input, should_succeed) in inputs {
            let result = cli::parse(input);
            if should_succeed {
                assert!(
                    result.is_ok(),
                    "Expected '{input}' to parse successfully, got {result:?}"
                );
            } else {
                assert!(
                    result.is_err(),
                    "Expected '{input}' to fail, got {result:?}"
                );
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if !args.is_empty() {
        let input = args.join(" ");

        match cli::parse(&input) {
            Ok(command) => runner::dispatch(command),
            Err(err) => eprintln!("Error: {err}"),
        }

        return;
    }

    // Interactive mode
    println!("KFI Interactive Mode");
    println!("Type 'exit' or 'quit' to leave.\n");

    let stdin = io::stdin();

    loop {
        print!("\n> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();

        if stdin.read_line(&mut line).is_err() {
            eprintln!("Failed to read input.");
            continue;
        }

        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        if matches!(line, "exit" | "quit") {
            break;
        }

        match cli::parse(line) {
            Ok(command) => runner::dispatch(command),
            Err(err) => eprintln!("Error: {err}"),
        }
    }
}
