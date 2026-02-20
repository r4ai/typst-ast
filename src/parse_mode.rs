use serde::Deserialize;

#[derive(Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ParseMode {
    #[default]
    Markup,
    Code,
    Math,
}
