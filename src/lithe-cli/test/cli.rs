#[cfg(test)]
mod test {
    use std::env;
    use std::error::Error;

    use assert_cmd::Command;
    use predicates::prelude::*;

    const BIN_NAME: &str = "lithe";

    #[test]
    fn test_help() -> Result<(), Box<dyn Error>> {
        for flag in &["-h", "--help"] {
            Command::cargo_bin(BIN_NAME)?
                .arg(flag)
                .assert()
                .success()
                .stdout(predicate::str::contains("Usage"));
        }
        Ok(())
    }

    #[test]
    fn test_version() -> Result<(), Box<dyn Error>> {
        let version = env::var("CARGO_PKG_VERSION")?;
        let expected = &format!("Lithe CLI {}", version);

        for flag in &["-V", "--version"] {
            Command::cargo_bin(BIN_NAME)?
                .arg(flag)
                .assert()
                .success()
                .stdout(predicate::str::contains(expected));
        }
        Ok(())
    }

    #[ignore]
    #[test]
    fn test_read_stdin() -> Result<(), Box<dyn Error>> {
        let expected = "<!DOCTYPE html>";

        Command::cargo_bin(BIN_NAME)?
            .args(["-"])
            .write_stdin("doctype html")
            .assert()
            .success()
            .stdout(expected);
        Ok(())
    }

    #[test]
    fn test_read_empty_file() -> Result<(), Box<dyn Error>> {
        let expected = "";

        Command::cargo_bin(BIN_NAME)?
            .args(["test/data/empty.slim"])
            .assert()
            .success()
            .stdout(expected);
        Ok(())
    }

    #[test]
    fn test_read_nonexistent_file() -> Result<(), Box<dyn Error>> {
        let expected = "No such file or directory (os error 2)";

        Command::cargo_bin(BIN_NAME)?
            .args(["test/data/nonexistent.slim"])
            .assert()
            .failure()
            .stderr(predicates::str::contains(expected));
        Ok(())
    }

    #[ignore]
    #[test]
    fn test_read_html_file() -> Result<(), Box<dyn Error>> {
        let expected = "<!DOCTYPE html>";

        Command::cargo_bin(BIN_NAME)?
            .args(["test/data/doctype-html.slim"])
            .assert()
            .success()
            .stdout(expected);
        Ok(())
    }
}
