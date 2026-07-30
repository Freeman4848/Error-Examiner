pub(crate) fn batches(input: &str, budget: usize) -> Vec<String> {
    let mut batches = Vec::new();
    let mut current = String::new();
    let mut current_chars = 0;
    for line in input.split_inclusive('\n') {
        let line_chars = line.chars().count();
        if !current.is_empty() && current_chars + line_chars > budget {
            batches.push(current);
            current = String::new();
            current_chars = 0;
        }
        if line_chars > budget {
            for character in line.chars() {
                if current_chars == budget {
                    batches.push(current);
                    current = String::new();
                    current_chars = 0;
                }
                current.push(character);
                current_chars += 1;
            }
        } else {
            current.push_str(line);
            current_chars += line_chars;
        }
    }
    if !current.is_empty() {
        batches.push(current);
    }
    batches
}
