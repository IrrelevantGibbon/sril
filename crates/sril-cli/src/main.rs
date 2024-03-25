use std::io::{self, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();
    let mut input = String::new();
    let mut env = sril::env::Env::default();
    write!(stdout, "Welcome to sril version 0.0.1 !\n\n")?;

    loop {
        write!(stdout, "→  ")?;
        stdout.flush()?;

        stdin.read_line(&mut input)?;

        match run(input.trim(), &mut env) {
            Ok(Some(val)) => writeln!(stdout, "{}", val)?,
            Ok(None) => {}
            Err(msg) => writeln!(stderr, "{}", msg)?,
        }
        input.clear();
    }
}

fn run(input: &str, env: &mut sril::env::Env) -> Result<Option<sril::value::Value>, String> {
    let parse = sril::parse(input).map_err(|msg| format!("Parse error: {}", msg))?;

    let evaluated = parse
        .eval(env)
        .map_err(|msg| format!("Evaluation error: {}", msg))?;

    if evaluated == sril::value::Value::Unit {
        Ok(None)
    } else {
        Ok(Some(evaluated))
    }
}
