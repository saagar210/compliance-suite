use crate::audit::canonical::CanonicalJson;
use crate::domain::errors::{CoreError, CoreErrorCode, CoreResult};
use crate::util::json::JsonValue;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestFile {
    pub path: String,
    pub sha256: String,
    pub size: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportManifest {
    pub version: i64,
    pub files: Vec<ManifestFile>,
}

impl ExportManifest {
    pub fn to_json_string(&self) -> String {
        let mut root = CanonicalJson::object();
        root.insert("version", CanonicalJson::Number(self.version));

        let mut files = Vec::new();
        for f in &self.files {
            let mut o = CanonicalJson::object();
            o.insert("path", CanonicalJson::String(f.path.clone()));
            o.insert("sha256", CanonicalJson::String(f.sha256.clone()));
            o.insert("size", CanonicalJson::Number(f.size));
            files.push(o);
        }
        root.insert("files", CanonicalJson::Array(files));
        root.encode()
    }

    pub fn from_json_str(s: &str) -> CoreResult<Self> {
        let v = JsonValue::parse(s)?;
        let obj = v.as_object()?;

        let version = obj.get_i64("version")?;

        let files_v = obj
            .get("files")
            .ok_or_else(|| CoreError::new(CoreErrorCode::CorruptVault, "manifest missing files"))?;
        let arr = files_v.as_array()?;

        let mut files = Vec::new();
        for item in arr {
            let o = item.as_object()?;
            files.push(ManifestFile {
                path: o.get_string("path")?,
                sha256: o.get_string("sha256")?,
                size: o.get_i64("size")?,
            });
        }

        Ok(ExportManifest { version, files })
    }
}
