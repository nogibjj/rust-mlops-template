use assert_cmd::Command;

#[test]
//a test to invoke the cli with an subcommand 'play' and option --name Marco
fn marco() {
    let mut cmd = Command::cargo_bin("marco_polo").unwrap();
    cmd.arg("play").arg("--name").arg("Marco");
    cmd.assert().success().stdout("Polo\n");

}

#[test]
//a test to invoke the cli with an subcommand 'play' and option --name Bob
fn polo() {
    let mut cmd = Command::cargo_bin("marco_polo").unwrap();
    cmd.arg("play").arg("--name").arg("Bob");
    cmd.assert().success().stdout("Marco\n");
}

