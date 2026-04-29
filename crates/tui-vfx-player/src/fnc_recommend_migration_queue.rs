// <FILE>crates/tui-vfx-player/src/fnc_recommend_migration_queue.rs</FILE> - <DESC>Build conservative migration queue</DESC>
// <VERS>VERSION: 0.1.1</VERS>
// <WCTX>New kernel Phase K2.2 review: keep recommendation wording phase-neutral.</WCTX>
// <CLOG>0.1.1: PATCH — remove phase labels from public recommendation rationales.
// 0.1.0: INIT — split queue recommendations from report construction.</CLOG>

use crate::PlayerMigrationQueueItem;

/// Return the provisional, conservative migration/adaptation queue for migration planning.
pub(crate) fn recommended_queue() -> Vec<PlayerMigrationQueueItem> {
    [
        ("complex", "create a minimal v3.1 complex fixture", "exercise mask + sampler + filter + shader + style/source after inventory evidence"),
        ("primitive-adapters", "clear remaining primitive adapter blockers", "reduce the six unsupported primitive ids before broad recipe migration"),
        ("content", "add a content family pilot", "content is legacy-present and v3.1-unrepresented but can start with path/descriptor inventory"),
        ("scene", "add a scene family pilot", "scene coverage is absent and should stay small until scene semantics are confirmed"),
        ("shadows", "add a shadow family pilot", "shadow migration needs descriptor decisions before broad parity claims"),
        ("complex", "choose complex legacy replacement candidates", "legacy complex coverage is large and should follow a minimal canonical fixture"),
        ("signals/easings/motion_routes", "settle timing and signal semantics", "these families need schema-level decisions before mechanical migration"),
        ("subcell_shapes/loopback/other", "audit advanced families", "advanced or ambiguous families need owner review after core coverage improves"),
    ]
    .into_iter()
    .enumerate()
    .map(|(index, (family, objective, rationale))| queue_item(index + 1, family, objective, rationale))
    .collect()
}

fn queue_item(
    rank: usize,
    family: &str,
    objective: &str,
    rationale: &str,
) -> PlayerMigrationQueueItem {
    PlayerMigrationQueueItem {
        rank,
        family: family.to_string(),
        objective: objective.to_string(),
        rationale: rationale.to_string(),
    }
}

// <FILE>crates/tui-vfx-player/src/fnc_recommend_migration_queue.rs</FILE> - <DESC>Build conservative migration queue</DESC>
// <VERS>END OF VERSION: 0.1.1</VERS>
