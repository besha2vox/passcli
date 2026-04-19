use dialoguer::{Input, Select};
use rand::random_range;

pub fn choose_pass_options() -> (&'static str, usize, bool) {
    let case_options = &["lower", "upper", "mixed"];
    let case_index = Select::new()
        .with_prompt("Select case")
        .items(case_options)
        .default(2)
        .interact()
        .unwrap();
    let selected_case = case_options[case_index];

    let length_input: String = Input::new()
        .with_prompt("Specify password length (e.g., 16 or 8-20)")
        .interact_text()
        .unwrap();

    let length = parse_length(&length_input);

    let use_specials: String = Input::new()
        .with_prompt("Include special characters? (y/n)")
        .interact_text()
        .unwrap();

    let use_specials = matches!(use_specials.to_lowercase().as_str(), "y");

    (selected_case, length, use_specials)
}

fn parse_length(input: &str) -> usize {
    if let Some((a, b)) = input.split_once('-') {
        let mut min = a.trim().parse::<usize>().unwrap_or(8);
        let mut max = b.trim().parse::<usize>().unwrap_or(16);
        if min > max {
            std::mem::swap(&mut min, &mut max);
        }
        random_range(min..=max)
    } else {
        input.trim().parse::<usize>().unwrap_or(16)
    }
}

pub fn generate_password(length: usize, case: &str, use_specials: bool) -> String {
    let mut charset = match case {
        "lower" => "abcdefghijklmnopqrstuvwxyz".to_string(),
        "upper" => "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string(),
        "mixed" => "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string(),
        _ => "abcdefghijklmnopqrstuvwxyz".to_string(),
    };

    charset.push_str("0123456789");

    if use_specials {
        charset.push_str("!@#$%^&*()_+-=[]{}|;:,.<>/?");
    }

    (0..length)
        .map(|_| {
            let idx = random_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect()
}
