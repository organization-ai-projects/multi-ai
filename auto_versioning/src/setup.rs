use crate::{
    cli::CliArgs,
    errors::Result,
    brain::{BrainLearner, decision::DecisionTreeLearner},
    api::ApiState,
    watcher::WatcherConfig,
    graph::DependencyGraph,
    init::{first_run_setup, save_learning_mode},
    backup, webhook, snapshot, config, git, cache,
};
use std::sync::{Arc, Mutex as StdMutex};
use tokio::sync::{broadcast, Mutex as TokioMutex};
use tracing::{debug, info};
use crate::watcher::VersionWatcher; // Ajouter cet import
use crate::init::LearningMode; // Ajouter cet import
use crate::api; // Ajouter cet import
use std::path::Path; // Ajouter cet import
use crate::commands::CommandRunner; // Ajouter cet import

pub struct AppComponents {
    pub brain: Arc<BrainLearner>,
    pub watcher_config: WatcherConfig,
    pub api_state: ApiState,
    pub graphs: AppGraphs,
}

pub struct AppGraphs {
    pub main_graph: Arc<StdMutex<DependencyGraph>>,
    pub api_graph: Arc<TokioMutex<DependencyGraph>>,
}

#[derive(Clone)]
pub struct CoreComponents {
    pub cargo_backup: Arc<StdMutex<backup::CargoBackup>>,
    pub command_runner: Arc<CommandRunner>,
    pub snapshot_manager: Arc<StdMutex<snapshot::SnapshotManager>>,
    pub webhook: Arc<webhook::WebhookManager>,
    pub project_config: config::AutoVersionConfig,
    pub git_manager: Option<git::GitManager>,
    pub project_cache: cache::ProjectCache, // Utiliser directement ProjectCache depuis cache.rs
    pub decision_tree: Arc<DecisionTreeLearner>,
}

fn setup_core_components(project_abs: &std::path::Path, args: &CliArgs) -> Result<CoreComponents> {
    // Base components
    let cargo_backup = Arc::new(StdMutex::new(
        backup::CargoBackup::new(&project_abs.join("Cargo.toml"))?
    ));
    let command_runner = Arc::new(CommandRunner::new(project_abs));
    let snapshot_manager = Arc::new(StdMutex::new(
        snapshot::SnapshotManager::new(&project_abs.join(".snapshots"))
    ));
    let webhook = Arc::new(webhook::WebhookManager::new(
        args.discord_webhook.as_deref(),
        args.github_webhook.as_deref()
    ));

    // Configuration et cache
    let project_config = config::AutoVersionConfig::load(project_abs);
    let git_manager = if !args.skip_git && project_config.git_enabled {
        Some(git::GitManager::new(project_abs))
    } else {
        None
    };
    let cache_dir = project_abs.join(".av-cache");
    let project_cache = crate::cache::ProjectCache::new(&cache_dir);

    // Machine Learning
    let decision_tree = Arc::new(DecisionTreeLearner::new());

    Ok(CoreComponents {
        cargo_backup,
        command_runner,
        snapshot_manager,
        webhook,
        project_config,
        git_manager,
        project_cache,
        decision_tree,
    })
}

fn create_watcher_config(
    args: &CliArgs,
    project_abs: &Path,
    core: &CoreComponents,
    brain: &Arc<BrainLearner>,
    graphs: &AppGraphs,
    updates_tx: &broadcast::Sender<api::VersionUpdate>,
) -> Result<WatcherConfig> {
    Ok(WatcherConfig {
        dry_run: args.dry_run,
        manual_bump: args.manual_bump.clone(),
        git_manager: core.git_manager.clone(),
        project_config: core.project_config.clone(),
        publish_target: args.publish_target.as_ref().cloned().map(|t| t.into()),
        cargo_backup: Some(core.cargo_backup.clone()),
        webhook: Some(core.webhook.clone()),
        snapshot_manager: Some(core.snapshot_manager.clone()),
        command_runner: Some(core.command_runner.clone()),
        project_cache: core.project_cache.clone(),
        auto_commit: args.auto_commit,
        log_level: args.log_level,
        dep_graph: Some(graphs.main_graph.clone()),
        updates_tx: Some(updates_tx.clone()),
        brain: Some(brain.as_ref().clone()),
        decision_tree: Some(core.decision_tree.as_ref().clone()),
        feedback_enabled: !args.dry_run,
        cargo_path: project_abs.join("Cargo.toml"),
        root_path: project_abs.to_path_buf(),
        post_publish: Vec::new(),
    })
}

fn create_api_state(
    graphs: &AppGraphs,
    updates_tx: &broadcast::Sender<api::VersionUpdate>,
) -> Result<ApiState> {
    Ok(ApiState {
        graph: graphs.api_graph.clone(),
        watcher: Arc::new(StdMutex::new(VersionWatcher::new(WatcherConfig::default()))),
        updates_tx: updates_tx.clone(),
    })
}

#[derive(Clone)]
pub struct ProjectCache {
    // ...existing fields...
}

async fn setup_learning_system(project_abs: &std::path::Path) -> Result<Arc<BrainLearner>> {
    let config_path = project_abs.join(".av-learning-mode");
    let learning_mode = if !config_path.exists() {
        let mode = first_run_setup();
        save_learning_mode(&mode, &project_abs.to_path_buf())?; // Convertir en PathBuf
        mode
    } else {
        let content = std::fs::read_to_string(&config_path)?;
        ron::from_str(&content)?
    };

    let brain = match learning_mode {
        LearningMode::Local => BrainLearner::new_local(&project_abs.join(".av-brain")),
        LearningMode::Collaborative => BrainLearner::new_collaborative(&project_abs.join(".av-brain")),
    };

    Ok(Arc::new(brain))
}

pub async fn initialize(args: CliArgs) -> Result<AppComponents> {
    let project_abs = args.target_project.canonicalize()?;
    
    // Initialiser les graphes
    let graphs = AppGraphs {
        main_graph: Arc::new(StdMutex::new(DependencyGraph::new())),
        api_graph: Arc::new(TokioMutex::new(DependencyGraph::new())),
    };

    let core = setup_core_components(&project_abs, &args)?;
    let brain = setup_learning_system(&project_abs).await?;
    let (updates_tx, _) = broadcast::channel(100);
    
    let watcher_config = create_watcher_config(
        &args, &project_abs, &core, &brain, &graphs, &updates_tx
    )?;
    
    let api_state = create_api_state(&graphs, &updates_tx)?;

    Ok(AppComponents {
        brain,
        watcher_config,
        api_state,
        graphs,
    })
}
