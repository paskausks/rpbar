use std::fmt::Display;

pub enum Markup {
    None,
    Pango,
}

impl Display for Markup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Markup::None => "none",
            Markup::Pango => "pango",
        })
    }
}

pub struct Status<'a>{
    pub name: &'a str,
    pub full_text: String,
    pub short_text: String,
    pub markup: Markup,
}

impl<'a> Status<'a> {
    pub fn to_json(&self) -> String {
        return format!(
            "{{\"name\": \"{}\",\"full_text\": \"{}\",\"short_text\":\"{}\",\"markup\": \"{}\"}}",
            self.name,
            self.full_text.replace("\\", "\\\\").replace("\"", "\\\""),
            self.short_text.replace("\\", "\\\\").replace("\"", "\\\""),
            self.markup,
        );
    }
}

pub trait Plugin {
    fn setup(&mut self);
    fn update(&mut self);
    fn get_status(&self) -> Option<Status>;
}
