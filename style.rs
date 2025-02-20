use crate::Style;
pub fn generate_colored_text(text:impl ToString, color:&str) -> console::StyledObject<String> {
    let colored_text = match color {
        "green" => Style::new().green().bold().apply_to(text.to_string()),
        "red" => Style::new().red().bold().apply_to(text.to_string()),
        "yellow" => Style::new().yellow().bold().apply_to(text.to_string()),
        _ => Style::new().apply_to(text.to_string()),
    };
    colored_text
}
