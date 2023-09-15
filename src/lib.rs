#![feature(macro_metavar_expr)]
use anyhow::{anyhow, ensure, Context, Result};
use regex::Regex;
use shallot::*;
use shallot_strings::LispString;

pub fn re_match<E>(arguments: &[E], _env: &mut Environment<E>) -> Result<E>
where
    E: LispExpression + ToAndFrom<LispString>,
{
    ensure!(
        arguments.len() == 2,
        "Function match must be called with two arguments"
    );

    let needle: &LispString = arguments[0]
        .try_into_atom()
        .context("Needle must be a string")?;
    let haystack: &LispString = arguments[1]
        .try_into_atom()
        .context("Needle must be a string")?;
    let re = Regex::new(&needle.0).with_context(|| anyhow!("Not a valid regex: {}", needle))?;
    Ok(List(
        re.find_iter(&haystack.0)
            .map(|mtch| E::from(LispString(mtch.as_str().to_owned())))
            .collect::<Vec<_>>(),
    )
    .into())
}

create_layer!(
   over shallot_strings
   | atoms
   | builtins "match" -> BuiltinFunction::new("match", re_match)
);
