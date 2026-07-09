fn main() {
    let new_version = semver_planner::bump_from_snapshots();
    println!("📦 Nouvelle version : {}", new_version.to_string());
}
