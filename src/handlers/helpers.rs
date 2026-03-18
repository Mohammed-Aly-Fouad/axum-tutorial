pub fn extract_error<F>(input: &str, mut f: F)
where
    F: FnMut(String, String),
{
    let lines = input.lines();

    lines.for_each(|line| {
        if let Some((first, second)) = line.split_once(": ") {
            f(first.trim().to_string(), second.trim().to_string());
        };
    });
}
