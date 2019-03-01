use crate::{
    lib::{
        ghrtask::GHRTask,
        task::{self, Task},
    },
    utils::{
        github::Asset,
        golang::{arch, os},
    },
};

pub fn task() -> Task {
    Task {
        name: "minikube".to_string(),
        sync,
        update,
    }
}

const GHR_TASK: GHRTask = GHRTask {
    asset_filter,
    command: "minikube",
    repo: ("kubernetes", "minikube"),
    trim_version,
    version_arg: "version",
};

fn asset_filter(asset: &Asset) -> bool {
    let name = format!("minikube-{}-{}", os(), arch());

    asset.name == name
}

fn sync() -> task::Result {
    GHR_TASK.sync()
}

#[allow(clippy::needless_pass_by_value)]
fn trim_version(stdout: String) -> String {
    let line = stdout.lines().next().unwrap_or_default();
    let parts: Vec<&str> = line.splitn(2, ':').collect();
    if parts.len() == 2 {
        return String::from(parts[1].trim());
    }
    String::from("unexpected")
}

fn update() -> task::Result {
    GHR_TASK.update()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_version_output() {
        let stdout = String::from("minikube version: v0.28.2\n");
        let got = trim_version(stdout);
        assert_eq!(got, String::from("v0.28.2"));
    }
}
