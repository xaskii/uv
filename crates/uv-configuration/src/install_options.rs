use tracing::debug;

use uv_pep508::PackageName;

#[derive(Debug, Clone, Default)]
pub struct InstallOptions {
    /// Omit the project itself from the resolution.
    pub no_install_project: bool,
    /// Omit all workspace members (including the project itself) from the resolution.
    pub no_install_workspace: bool,
    /// Omit all local packages from the resolution.
    pub no_install_local: bool,
    /// Omit the specified packages from the resolution.
    pub no_install_package: Vec<PackageName>,
}

impl InstallOptions {
    pub fn new(
        no_install_project: bool,
        no_install_workspace: bool,
        no_install_local: bool,
        no_install_package: Vec<PackageName>,
    ) -> Self {
        Self {
            no_install_project,
            no_install_workspace,
            no_install_local,
            no_install_package,
        }
    }

    /// Returns `true` if a package passes the install filters.
    pub fn include_package(
        &self,
        package_name: &uv_pep508::PackageName,
        is_local: impl FnOnce() -> bool,
        project_name: Option<&uv_pep508::PackageName>,
        members: &std::collections::BTreeSet<uv_pep508::PackageName>,
    ) -> bool {
        // If `--no-install-project` is set, remove the project itself.
        if self.no_install_project {
            if let Some(project_name) = project_name {
                if package_name == project_name {
                    debug!(
                        "Omitting `{}` from resolution due to `--no-install-project`",
                        package_name
                    );
                    return false;
                }
            }
        }

        // If `--no-install-workspace` is set, remove workspace members.
        if self.no_install_workspace {
            if members.contains(package_name) {
                debug!(
                    "Omitting `{}` from resolution due to `--no-install-workspace`",
                    package_name
                );
                return false;
            }
        }

        // If `--no-install-local` is set, remove local packages.
        if self.no_install_local {
            if is_local() {
                debug!(
                    "Omitting `{}` from resolution due to `--no-install-local`",
                    package_name
                );
                return false;
            }
        }

        // If `--no-install-package` is provided, remove the requested packages.
        if self.no_install_package.contains(package_name) {
            debug!(
                "Omitting `{}` from resolution due to `--no-install-package`",
                package_name
            );
            return false;
        }

        true
    }
}
