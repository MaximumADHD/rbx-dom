use std::fs;
use std::process::Command;

use anyhow::Context;
use roblox_install::RobloxStudio;
use serde::Deserialize;
use tempfile::tempdir;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Dump {
    pub classes: Vec<DumpClass>,
    pub enums: Vec<DumpEnum>,
}

impl Dump {
    pub fn read() -> anyhow::Result<Dump> {
        let studio_install =
            RobloxStudio::locate().context("Could not locate Roblox Studio install")?;

        let dir = tempdir()?;
        let dump_path = dir.path().join("api-dump.json");

        Command::new(studio_install.application_path())
            .arg("-API")
            .arg(&dump_path)
            .status()?;

        let contents = fs::read_to_string(&dump_path).context("Could not read API dump")?;
        let dump: Dump = serde_json::from_str(&contents)
            .context("Roblox Studio produced an invalid API dump")?;

        Ok(dump)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DumpClass {
    pub name: String,
    pub superclass: String,

    #[serde(default)]
    pub tags: Vec<String>,
    pub members: Vec<DumpClassMember>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "MemberType")]
pub enum DumpClassMember {
    Property(DumpClassProperty),

    #[serde(rename_all = "PascalCase")]
    Function {
        name: String,
    },

    #[serde(rename_all = "PascalCase")]
    Event {
        name: String,
    },

    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DumpClassProperty {
    pub name: String,
    pub value_type: ValueType,
    pub serialization: Serialization,
    pub security: PropertySecurity,

    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ValueType {
    pub name: String,
    pub category: ValueCategory,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum ValueCategory {
    /// Lua primitives like float or string
    Primitive,

    /// Roblox data types like Vector3 or CFrame
    DataType,

    /// Roblox enum like FormFactor or Genre
    Enum,

    /// An instance reference
    Class,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[allow(clippy::enum_variant_names)]
pub enum Security {
    None,
    LocalUserSecurity,
    PluginSecurity,
    RobloxScriptSecurity,
    NotAccessibleSecurity,
    RobloxSecurity,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PropertySecurity {
    pub read: Security,
    pub write: Security,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Serialization {
    pub can_save: bool,
    pub can_load: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DumpEnum {
    pub name: String,
    pub items: Vec<DumpEnumItem>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DumpEnumItem {
    pub name: String,
    pub value: u32,
}
