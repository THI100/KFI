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
            ("Change fanf", true),
            ("Del mac", true),
            ("branch a -N", true),
            ("checkout b", true),
            ("fuse a b -i", true),
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
    let k = "init fub ./";

    let j = cli::parse(k);

    print!("{:?}", j)
}
