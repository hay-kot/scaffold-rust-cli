//! End-to-end tests that run the compiled binary.

#[cfg(test)]
mod cli {
    use assert_cmd::Command;
    use predicates::str::contains;

    fn bin() -> Command {
        Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
    }

    #[test]
    fn help_lists_commands() {
        bin()
            .arg("--help")
            .assert()
            .success()
{{- range .Scaffold.commands }}
            .stdout(contains("{{ . }}"))
{{- end }}
            .stdout(contains("completions"));
    }

    #[test]
    fn version_prints_crate_version() {
        bin().arg("--version").assert().success().stdout(contains(env!("CARGO_PKG_VERSION")));
    }
{{- range .Scaffold.commands }}

    #[test]
    fn {{ . | toSnakeCase }}_prints_greeting() {
        bin().args(["--no-color", "{{ . }}"]).assert().success().stdout("Hello World!\n");
    }
{{- end }}

    #[test]
    fn unknown_command_fails() {
        bin().arg("not-a-command").assert().failure();
    }

    #[test]
    fn completions_generates_a_script() {
        bin().args(["completions", "bash"]).assert().success().stdout(contains("{{ .ProjectKebab }}"));
    }
}
