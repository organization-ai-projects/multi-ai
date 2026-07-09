use serde::{Deserialize, Serialize};

// Gère une collection générique d'éléments (utilisé par le manager pour les nœuds du graphe mémoire de l'IA).

#[derive(Debug, Default, Serialize, Deserialize)]
pub(crate) struct MemoryNodes<T> { // Visibilité restreinte à `memory`
    nodes: Vec<T>,
}

impl<T> MemoryNodes<T> {
    pub(crate) fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    /// Ajoute un nœud à la fin de la collection.
    pub(crate) fn push_node(&mut self, node: T) {
        self.nodes.push(node);
    }

    /// Ajoute plusieurs nœuds à la fin de la collection.
    pub(crate) fn push_nodes<I: IntoIterator<Item = T>>(&mut self, nodes: I) {
        self.nodes.extend(nodes);
    }

    /// Supprime le premier nœud correspondant au prédicat.
    pub(crate) fn remove_first_node<F>(&mut self, mut predicate: F) -> Option<T>
    where
        F: FnMut(&T) -> bool,
    {
        if let Some(pos) = self.nodes.iter().position(|n| predicate(n)) {
            Some(self.nodes.remove(pos))
        } else {
            None
        }
    }

    pub(crate) fn remove_nodes<F>(&mut self, mut predicate: F) -> Vec<T>
    where
        F: FnMut(&T) -> bool,
    {
        let mut removed = Vec::new();
        let mut i = 0;
        while i < self.nodes.len() {
            if predicate(&self.nodes[i]) {
                removed.push(self.nodes.remove(i));
            } else {
                i += 1;
            }
        }
        removed
    }

    /// Retourne une référence à tous les éléments de la collection.
    pub(crate) fn as_slice(&self) -> &[T] {
        &self.nodes
    }

    /// Applique une fonction à chaque élément de la collection (lecture seule).
    pub(crate) fn foreach(&self, mut f: impl FnMut(&T)) {
        for item in &self.nodes {
            f(item);
        }
    }

    /// Applique une fonction à chaque élément de la collection (pour modification).
    pub(crate) fn foreach_for_update(&mut self, mut f: impl FnMut(&mut T)) {
        for item in &mut self.nodes {
            f(item);
        }
    }

    /// Trouve le premier élément correspondant au prédicat (lecture seule).
    pub(crate) fn find_first(&self, mut predicate: impl FnMut(&T) -> bool) -> Option<&T> {
        self.nodes.iter().find(|n| predicate(n))
    }

    /// Trouve le premier élément correspondant au prédicat (pour modification).
    pub(crate) fn find_first_for_update(
        &mut self,
        mut predicate: impl FnMut(&T) -> bool,
    ) -> Option<&mut T> {
        self.nodes.iter_mut().find(|n| predicate(n))
    }

    /// Trouve le premier nœud correspondant au prédicat (lecture seule).
    pub(crate) fn get_node<F>(&self, mut predicate: F) -> Option<&T>
    where
        F: FnMut(&T) -> bool,
    {
        self.nodes.iter().find(|n| predicate(n))
    }

    pub(crate) fn all(&self) -> &Vec<T> {
        &self.nodes
    }
}
