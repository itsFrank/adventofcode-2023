use std::fs;

#[macro_export]
macro_rules! resource_path {
    ($res_path:expr) => {{
        use std::path::Path;
        let path = Path::new(env!("CARGO_MANIFEST_DIR"));
        match path.join("resources").join($res_path).to_str() {
            Some(it) => Some(it.to_string()),
            None => None,
        }
    }};
}

pub fn read_file(file_path: &str) -> Result<String, String> {
    let contents = fs::read_to_string(&file_path);
    return match contents {
        Ok(c) => Ok(c),
        Err(r) => Err(format!("path: {} - {}", file_path, r.to_string())),
    };
}

pub fn get_resource_lines(res_path: &str) -> Vec<String> {
    let mut lines: Vec<String> = read_file(&res_path)
        .unwrap()
        .split("\n")
        .map(|s| s.to_string())
        .collect();

    // remove last line if empty
    if let Some(line) = lines.last() {
        if line.is_empty() {
            lines.pop();
        }
    }

    return lines;
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_resource_path() {
        let exp_suffix = "aoc_shared/resources/test/file.json";

        let path = resource_path!("test/file.json").unwrap();
        let got_suffix = {
            let split_pos = path
                .char_indices()
                .nth_back(exp_suffix.len() - 1)
                .unwrap()
                .0;
            &path[split_pos..]
        };

        assert_eq!(exp_suffix, got_suffix);
    }

    #[test]
    fn test_read_resource_file() {
        let path = resource_path!("sample.txt").unwrap();
        let got_content = read_file(&path).unwrap();
        assert_eq!("hello resource!\n", got_content);
    }
}
