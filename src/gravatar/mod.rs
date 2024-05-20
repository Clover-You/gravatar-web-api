use serde::{Deserialize, Deserializer, Serialize};

use crate::serde_visitor::bool_form_string::StringOrBoolVisitor;

pub const GRAVATAR_URL: &'static str = "https://gravatar.com/";

#[derive(Deserialize, Debug, Serialize)]
pub struct GravatarEntry {
  pub entry: Vec<Gravatar>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Gravatar {
  pub hash: String,
  #[serde(rename(deserialize = "requestHash"))]
  pub request_hash: String,
  #[serde(rename(deserialize = "profileUrl"))]
  pub profile_url: String,
  #[serde(rename(deserialize = "preferredUsername"))]
  pub preferred_username: String,
  #[serde(rename(deserialize = "thumbnailUrl"))]
  pub thumbnail_url: String,
  pub photos: Vec<Photos>,
  pub last_profile_edit: String,
  #[serde(rename(deserialize = "displayName"))]
  pub display_name: String,
  pub pronunciation: String,
  pub pronouns: String,
  #[serde(rename(deserialize = "aboutMe"))]
  pub about_me: String,
  #[serde(rename(deserialize = "currentLocation"))]
  pub current_location: String,
  pub job_title: String,
  pub company: String,
  pub emails: Vec<Email>,
  pub name: Name,
  pub full_name: String,
  pub accounts: Vec<Account>,
  pub urls: Vec<Link>,
  #[serde(rename(deserialize = "profileBackground"))]
  pub profile_background: ProfileBackground,
  pub share_flags: ShareFlags,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct ShareFlags {
  pub search_engines: bool,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct ProfileBackground {
  pub opacity: i8,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Link {
  pub title: String,
  pub value: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Account {
  pub domain: String,
  pub display: String,
  pub url: String,
  #[serde(rename(deserialize = "iconUrl"))]
  pub icon_url: String,
  pub username: String,
  pub verified: String,
  pub name: String,
  pub shortname: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Name {
  #[serde(rename(deserialize = "givenName"))]
  pub given_name: String,
  #[serde(rename(deserialize = "familyName"))]
  pub family_name: String,
  pub formatted: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Email {
  #[serde(deserialize_with = "bool_from_string")]
  pub primary: bool,
  pub value: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Photos {
  pub value: String,
  pub r#type: String,
}

pub fn bool_from_string<'de, D>(deserialize: D) -> Result<bool, D::Error>
where
  D: Deserializer<'de>,
{
  deserialize.deserialize_any(StringOrBoolVisitor)
}
