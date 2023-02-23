use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub struct CommitInfo {
    sha: String,
    node_id: String,
    commit: Commit,
    url: String,
    html_url: String,
    comments_url: String,
    author: AuthorDetails,
    committer: Commiter,
    parents: Vec<Tree>,
    stats: Stats,
    files: Vec<CommitFile>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub struct Commit {
    author: Author,
    committer: Author,
    message: String,
    tree: Tree,
    url: String,
    comment_count: u64,
    verification: Verification,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub struct Author {
    name: String,
    email: String,
    date: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub struct Tree {
    sha: String,
    url: String,
    html_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub struct Verification {
    verified: bool,
    reason: String,
    signature: String,
    payload: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Default)]
pub enum VerificationReason {
    Valid,
    #[default]
    Invalid,
    CatchAll,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub struct AuthorDetails {
    login: String,
    id: u64,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    r#type: String,
    site_admin: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub struct Commiter {
    login: String,
    id: u64,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    r#type: String,
    site_admin: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub struct Stats {
    total: u64,
    additions: u64,
    deletions: u64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub struct CommitFile {
    sha: String,
    filename: String,
    status: String,
    additions: u64,
    deletions: u64,
    changes: u64,
    blob_url: String,
    raw_url: String,
    contents_url: String,
    patch: String,
}
