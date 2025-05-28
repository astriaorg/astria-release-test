const { CargoWorkspace } = require('release-please/dist/src/plugins/cargo-workspace');

/**
 * Custom Cargo Workspace plugin that ignores self-dependencies.
 * This handles the common Rust pattern where a crate depends on itself
 * as a dev-dependency to test features.
 */
class CargoWorkspaceNoSelfDeps extends CargoWorkspace {
  async buildGraph(allPackages) {
    const workspaceCrateNames = new Set(
      allPackages.map(crateInfo => crateInfo.name)
    );
    const graph = new Map();

    for (const crateInfo of allPackages) {
      const allDeps = Object.keys({
        ...(crateInfo.manifest.dependencies || {}),
        ...(crateInfo.manifest['dev-dependencies'] || {}),
        ...(crateInfo.manifest['build-dependencies'] || {}),
      });

      const targets = crateInfo.manifest.target;
      if (targets) {
        for (const targetName in targets) {
          const target = targets[targetName];
          allDeps.push(
            ...Object.keys({
              ...(target.dependencies || {}),
              ...(target['dev-dependencies'] || {}),
              ...(target['build-dependencies'] || {}),
            })
          );
        }
      }

      // Filter workspace dependencies, excluding self-dependencies
      const workspaceDeps = allDeps.filter(dep =>
        workspaceCrateNames.has(dep) &&
        dep !== crateInfo.name  // This is the key change - ignore self-dependencies
      );

      if (allDeps.includes(crateInfo.name)) {
        this.logger.info(
          `Ignoring self-dependency for ${crateInfo.name} (likely used for feature testing)`
        );
      }

      graph.set(crateInfo.name, {
        deps: workspaceDeps,
        value: crateInfo,
      });
    }

    return graph;
  }
}

// Export the class
exports.CargoWorkspaceNoSelfDeps = CargoWorkspaceNoSelfDeps;
