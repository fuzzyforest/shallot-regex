use anyhow::Result;
use shallot::*;

fn main() -> Result<()> {
    let mut environment: Environment<shallot_regex::Expression> = Environment::default();
    shallot::builtins::set_environment(&mut environment);
    shallot_regex::set_environment(&mut environment);
    run_repl(&mut environment)
}
