pub(crate) fn exclude<'a>(args: &Vec<&'a str>, remove_string: &'a str) -> Vec<&'a str> {
    args.iter()
        .filter(|x| !x.eq(&&remove_string))
        .copied()
        .collect()
}

pub(crate) fn limit_text(text: &str, max_width: usize) -> String {
    if text.len() <= max_width {
        return text.to_string();
    }
    // TODO what is c.dim?
    // return `${text.slice(0, maxWidth)}${c.dim('…')}`
    format!("{}…", &text[..max_width])
}

#[cfg(test)]
mod tests {
    use super::*;

    // exclude
    #[test]
    fn exclude_it_works() {
        let result = exclude(&vec!["npm", "i", "-g", "axios"], "-g");
        assert_eq!(result, vec!["npm", "i", "axios"]);
    }

    // limit_text
    #[test]
    fn limit_text_it_works() {
        let result = limit_text("abcdefghijklmnopqrstuvwxyz", 3);
        assert_eq!(result, "abc…");
    }
}
