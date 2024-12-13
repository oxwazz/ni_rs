pub(crate) fn exclude<'a>(args: &Vec<&'a str>, remove_string: &'a str) -> Vec<&'a str> {
    args.iter()
        .filter(|x| !x.eq(&&remove_string))
        .copied()
        .collect()
}
