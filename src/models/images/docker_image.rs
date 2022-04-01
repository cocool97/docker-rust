use serde::Deserialize;

/// Represents a Docker Image.
#[derive(Deserialize, Debug)]
#[serde(rename = "PascalCase")]
pub struct DockerImage {
    /// Image identifier.
    pub id: String,
    /// Parent image identifier.
    pub parent_id: String,
    /// Tags in the repository.
    pub repo_tags: Vec<String>,
    /// Digests in the repository.
    pub repo_digests: Vec<String>,
    /// Creation date.
    pub created: u64,
    /// Size (bytes).
    pub size: u64,
    /// Virtual size (bytes).
    pub virtual_size: u64,
    /// Shared size (bytes).
    pub shared_size: u64,
    /// Image labels.
    pub labels: ImageLabel,
    /// Number of linked containers.
    pub containers: u32,
}

/// Represents label sets on a docker image.
#[derive(Deserialize, Debug)]
#[serde(rename = "PascalCase")]
pub struct ImageLabel {
    /// Build version.
    pub build_version: String,
    /// Maintainer(s)
    pub maintainer: String,
}
