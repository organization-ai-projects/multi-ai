use serde::{Deserialize, Serialize};

// Gère une collection générique d'éléments (utilisé par le manager pour les liens du graphe mémoire de l'IA).

#[derive(Debug, Default, Serialize, Deserialize)]
pub(crate) struct MemoryLinks<T> { // Visibilité restreinte à `memory`
    links: Vec<T>,
}

impl<T> MemoryLinks<T> {
    pub(crate) fn new() -> Self {
        Self { links: Vec::new() }
    }

    /// Ajoute un lien à la fin de la collection.
    pub(crate) fn push_link(&mut self, link: T) {
        self.links.push(link);
    }

    /// Ajoute plusieurs liens à la fin de la collection.
    pub(crate) fn push_links<I: IntoIterator<Item = T>>(&mut self, links: I) {
        self.links.extend(links);
    }

    pub(crate) fn remove_first_link<F>(&mut self, mut predicate: F) -> Option<T>
    where
        F: FnMut(&T) -> bool,
    {
        if let Some(pos) = self.links.iter().position(|l| predicate(l)) {
            Some(self.links.remove(pos))
        } else {
            None
        }
    }

    pub(crate) fn remove_links<F>(&mut self, mut predicate: F) -> Vec<T>
    where
        F: FnMut(&T) -> bool,
    {
        let mut removed = Vec::new();
        let mut i = 0;
        while i < self.links.len() {
            if predicate(&self.links[i]) {
                removed.push(self.links.remove(i));
            } else {
                i += 1;
            }
        }
        removed
    }

    pub(crate) fn as_slice(&self) -> &[T] {
        &self.links
    }

    pub(crate) fn all(&self) -> &Vec<T> {
        &self.links
    }
}
