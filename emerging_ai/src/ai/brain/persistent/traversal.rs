use crate::brain::persistent::nodes::MemoryNode;
use crate::brain::persistent::links::MemoryLink;
use crate::brain::brain_api::BrainAPI;
use uuid::Uuid;

pub enum TraversalStrategy {
    Exhaustive,                // Parcours exhaustif de tous les nœuds et liens
    RecentChanges(u64),        // Parcours des nœuds et liens modifiés après un certain timestamp
    Targeted(Uuid),            // Parcours ciblé autour d'un nœud spécifique
    Hybrid(Vec<TraversalStrategy>), // Combinaison de plusieurs stratégies
}

impl TraversalStrategy {
    pub fn apply(&self, brain_api: &mut BrainAPI) -> std::io::Result<Vec<Uuid>> {
        match self {
            TraversalStrategy::Exhaustive => {
                let nodes = brain_api.list_all_nodes()?;
                let links = brain_api.list_all_links()?;
                Ok(nodes.into_iter().map(|n| n.uuid).chain(links.into_iter().map(|l| l.uuid)).collect())
            }
            TraversalStrategy::RecentChanges(timestamp) => {
                let nodes = brain_api.find_nodes_modified_after(*timestamp)?;
                let links = brain_api.list_all_links()?.into_iter()
                    .filter(|link| link.timestamp > *timestamp)
                    .collect::<Vec<_>>();
                Ok(nodes.into_iter().map(|n| n.uuid).chain(links.into_iter().map(|l| l.uuid)).collect())
            }
            TraversalStrategy::Targeted(node_uuid) => {
                let links = brain_api.find_links_by_node(*node_uuid)?;
                Ok(links.into_iter().map(|l| l.uuid).collect())
            }
            TraversalStrategy::Hybrid(strategies) => {
                let mut results = Vec::new();
                for strategy in strategies {
                    if let Ok(mut uuids) = strategy.apply(brain_api) {
                        results.append(&mut uuids);
                    }
                }
                Ok(results)
            }
        }
    }
}
