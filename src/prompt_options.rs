use dialoguer::{console::Term, theme::ColorfulTheme, Select};

// const COMMANDS: &'static [&str] = &["list", "add"];

pub fn show<'a>(options: &'a Vec<&str>) -> &'a str {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(options)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .expect("error");

    return options[selection.unwrap()];
}
