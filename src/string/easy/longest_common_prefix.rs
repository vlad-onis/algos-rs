fn get_shortest<'a>(strs: &'a Vec<String>) -> &'a String {
    let mut min = &strs[0];
    for word in strs {
        if word.len() < min.len() {
            min = word;
        }
    }

    min
}

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::new();
    }

    let shortest = get_shortest(&strs);
    let shortest = &shortest[0..shortest.len()];

    let mut lcp = shortest;

    for word in &strs {
        let mut upper_limit = lcp.len();
        while !word.starts_with(lcp) {
            if upper_limit == 0 {
                return String::from("");
            }
            upper_limit = lcp.len() - 1;

            lcp = &lcp[0..upper_limit];
        }
    }

    lcp.to_string()
}
