use std::fmt;
use std::fmt::{Display, Formatter};
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
struct BuildInfo {
    build_timestamp: String,
    branch: String,
    commit: String,
    version: String,
}

impl Display for BuildInfo {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_fmt(format_args!("version: {}\nbuildTimestamp: {}\ncommit: {}\nbranch: {}\n",
                                 self.version, self.build_timestamp, self.commit, self.branch))
    }
}


pub fn build_details(format: String) {
    let bi = BuildInfo {
        build_timestamp: env!("VERGEN_BUILD_TIMESTAMP").to_string(),
        branch: env!("VERGEN_GIT_BRANCH").to_string(),
        commit: env!("VERGEN_GIT_SHA_SHORT").to_string(),
        version: env!("VERGEN_GIT_SEMVER").to_string(),
    };

    if format == "json" {
        println!("{}", serde_json::to_string_pretty(&bi).unwrap_or_else(|error| {
            error.to_string()
        }));
    } else if format == "yaml" {
        println!("{}", serde_yaml::to_string(&bi).unwrap_or_else(|error| {
            error.to_string()
        }));
    } else {
        println!("{}", bi)
    }
}
