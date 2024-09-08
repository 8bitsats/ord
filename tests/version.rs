use super::*;

#[test]
fn version_flag_prints_version() {
  CommandBuilder::new("--version")
<<<<<<< HEAD
    .stdout_regex("ord-dogecoin .*\n")
    .run();
=======
    .stdout_regex("ord .*\n")
    .run_and_extract_stdout();
>>>>>>> 5c09dd6c38136a95370eb5274d23a38b59306bb8
}
