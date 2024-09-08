use {super::*, ord::subcommand::parse::Output, ord::Object};

#[test]
<<<<<<< HEAD
=======
fn name() {
  assert_eq!(
    CommandBuilder::new("parse a").run_and_deserialize_output::<Output>(),
    Output {
      object: Object::Integer(2099999997689999),
    }
  );
}

#[test]
>>>>>>> 5c09dd6c38136a95370eb5274d23a38b59306bb8
fn hash() {
  assert_eq!(
    CommandBuilder::new("parse 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef")
      .run_and_deserialize_output::<Output>(),
    Output {
      object: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
        .parse::<Object>()
        .unwrap(),
    }
  );
}

#[test]
fn unrecognized_object() {
  CommandBuilder::new("parse A")
    .stderr_regex(r#"error: .*: unrecognized object\n.*"#)
    .expected_exit_code(2)
    .run_and_extract_stdout();
}
