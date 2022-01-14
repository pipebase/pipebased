use crate::daemon::DaemonService;
use pipebased_common::{
    Daemon, DaemonConfig, PipeManager, PipeManagerConfig, RepositoryManager,
    RepositoryManagerConfig,
};

fn build_repository_manager(config: RepositoryManagerConfig) -> RepositoryManager {
    let pb_client_config = config.pb_client;
    let app_directory = config.app_directory;
    let catalogs_directory = config.catalogs_directory;
    RepositoryManager::builder()
        .app_directory(app_directory)
        .catalogs_directory(catalogs_directory)
        .pb_client(pb_client_config.into())
        .build()
}

fn build_pipe_manager(config: PipeManagerConfig) -> PipeManager {
    let workspace = config.workspace;
    PipeManager::builder().workspace(workspace).build()
}

fn build_daemon(config: DaemonConfig) -> Daemon {
    let pipe_manager_config = config.pipe;
    let repository_manager_config = config.repository;
    let pipe_manager = build_pipe_manager(pipe_manager_config);
    let repository_manager = build_repository_manager(repository_manager_config);
    Daemon::builder()
        .pipe_manager(pipe_manager)
        .repository_manager(repository_manager)
        .build()
}

pub fn bootstrap(config: DaemonConfig) -> DaemonService {
    let daemon = build_daemon(config);
    DaemonService::builder().daemon(daemon).build()
}
