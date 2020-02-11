use exitfailure::ExitFailure;

pub fn find_matches(content: &str, pattern: &str, mut out: impl std::io::Write) -> Result<(), ExitFailure> {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(out, "{}", line)?;
        }
    }
    Ok(())
}
