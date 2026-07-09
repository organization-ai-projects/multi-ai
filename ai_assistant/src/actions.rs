use semver_planner::bump_from_snapshots;
use version_watcher::store::revert_snapshot;

pub fn bump_version() {
    let _ = bump_from_snapshots();
}

pub fn revert_to(id: &str) {
    let _ = revert_snapshot(id);
}
