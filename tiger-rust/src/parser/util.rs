use crate::parser::ast::Exp;

impl Exp {
    pub fn to_json_string(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|err| err.to_string())
    }

    pub fn to_yaml_string(&self) -> Result<String, String> {
        let yaml_data = serde_json::from_str::<serde_yaml::Value>(
            &serde_json::to_string(self).map_err(|err| err.to_string())?,
        )
        .map_err(|err| err.to_string())?;
        serde_yaml::to_string(&yaml_data).map_err(|err| err.to_string())
    }
}
