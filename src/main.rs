//! Show all environment variables, or particular ones.
//!
//! Note that this is not currently customized for Windows, where it would be
//! more useful to sort and deduplicate case-insensitively and, most of all, to
//! always show the name with whatever specific casing it is represented with
//! in the environment.

fn print_one_var(name: &str, value: &str) {
    println!("{name}={value}");
}

fn print_all_vars() {
    let mut pairs: Vec<_> = std::env::vars().collect();
    pairs.sort_by(|(lhs_name, _), (rhs_name, _)| lhs_name.cmp(rhs_name));
    for (name, value) in &pairs {
        print_one_var(name, value);
    }
}

fn print_some_vars(mut names: Vec<String>) {
    names.sort();
    names.dedup();
    for name in &names {
        if let Ok(value) = std::env::var(name) {
            print_one_var(name, &value);
        }
    }
}

fn main() {
    let names: Vec<_> = std::env::args().skip(1).collect();
    if names.is_empty() {
        print_all_vars();
    } else {
        print_some_vars(names);
    }
}
