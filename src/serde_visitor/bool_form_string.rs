/// 自定义转换器 bool str -->> bool
pub struct StringOrBoolVisitor;

impl<'de> serde::de::Visitor<'de> for StringOrBoolVisitor {
  type Value = bool;

  fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
    formatter.write_str("a boolean or string representing a boolean")
  }

  fn visit_str<E>(self, str_val: &str) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    match str_val {
      "true" => Ok(true),
      _ => Ok(false),
    }
  }

  fn visit_bool<E>(self, bool_val: bool) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    Ok(bool_val)
  }
}
