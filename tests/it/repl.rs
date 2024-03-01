use std::io::{self, Cursor};

use lip::repl;

fn get_outputs(output: Cursor<Vec<u8>>) -> Vec<String> {
    let out = String::from_utf8(output.into_inner()).unwrap();
    out.split("lip> ")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

#[test]
fn repl_eval_primitive_succeed() -> io::Result<()> {
    let mut input = Cursor::new("T\n:exit".as_bytes());
    let mut output = Cursor::new(Vec::new());
    repl::run(&mut input, &mut output)?;
    assert_eq!(vec!["true"], get_outputs(output));
    Ok(())
}

#[test]
fn repl_env_command_succeed() -> io::Result<()> {
    let mut input = Cursor::new("(def x (& T T T))\n:env\n:exit".as_bytes());
    let mut output = Cursor::new(Vec::new());
    repl::run(&mut input, &mut output)?;
    assert_eq!(
        vec![
            "true",
            "Environment { data: {\"x\": Bool(true)}, outer: None }"
        ],
        get_outputs(output)
    );
    Ok(())
}
