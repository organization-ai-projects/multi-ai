use std::process::Command;
use crate::IaList;

pub fn launch_ias(list: &IaList) {
    for path in list.nature.iter().chain(&list.scientist).chain(&list.life_form) {
        Command::new("cargo")
            .args([
                "run", "--manifest-path",
                &format!("{}/Cargo.toml", path)
            ])
            .spawn()
            .unwrap();
    }
}
