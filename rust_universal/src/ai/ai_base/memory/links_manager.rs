// Gestionnaire global pour manipuler les liens du graphe mémoire de l'IA.

use super::link::MemoryLink;
use super::links::MemoryLinks;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Serialize, Deserialize)]
pub(crate) struct LinksManager { // Visibilité restreinte à `memory`
    pub(crate) links: MemoryLinks<MemoryLink>,
    tags: HashMap<(usize, usize), Vec<String>>, // Associe des tags aux liens par leurs IDs (from, to)
}

impl LinksManager {
    pub(crate) fn new() -> Self {
        Self {
            links: MemoryLinks::new(),
            tags: HashMap::new(),
        }
    }

    /// Ajoute un lien à partir de ses données.
    pub(crate) fn insert_link(&mut self, from: usize, to: usize, weight: f32) {
        let link = MemoryLink::new(from, to, weight);
        self.links.push_link(link);
    }

    /// Ajoute plusieurs liens à partir d'itérables de (from, to, weight).
    pub(crate) fn insert_links<I>(&mut self, links: I)
    where
        I: IntoIterator<Item = (usize, usize, f32)>,
    {
        let links = links
            .into_iter()
            .map(|(from, to, weight)| MemoryLink::new(from, to, weight));
        self.links.push_links(links);
    }

    /// Supprime un lien selon un prédicat (ex: par id).
    pub(crate) fn remove_link<F>(&mut self, predicate: F) -> bool
    where
        F: FnMut(&MemoryLink) -> bool,
    {
        self.links.remove_first_link(predicate).is_some()
    }

    /// Récupère tous les liens.
    pub(crate) fn all(&self) -> &Vec<MemoryLink> {
        self.links.all()
    }

    /// Ajoute un lien bidirectionnel entre deux nœuds.
    pub(crate) fn insert_bidirectional_link(&mut self, from: usize, to: usize, weight: f32) {
        self.insert_link(from, to, weight); // Lien direct
        self.insert_link(to, from, weight); // Lien inverse
    }

    /// Récupère le poids d'un lien entre deux nœuds.
    pub(crate) fn get_link_weight(&self, from: usize, to: usize) -> Option<f32> {
        self.links
            .all()
            .iter()
            .find(|link| link.from == from && link.to == to)
            .map(|link| link.weight)
    }

    /// Ajuste le poids d'un lien existant ou le crée si nécessaire.
    pub(crate) fn adjust_link_weight(&mut self, from: usize, to: usize, delta: f32) {
        if let Some(link) = self.links.all().iter_mut().find(|l| l.from == from && l.to == to) {
            link.weight += delta; // Ajuste le poids existant
        } else {
            self.insert_link(from, to, delta); // Crée un nouveau lien avec le poids `delta`
        }
    }

    /// Ajoute un lien avec des tags optionnels.
    pub(crate) fn insert_link_with_tags(
        &mut self,
        from: usize,
        to: usize,
        weight: f32,
        tags: Option<Vec<String>>,
    ) {
        self.insert_link(from, to, weight);

        if let Some(tags) = tags {
            self.tags.insert((from, to), tags);
        }
    }

    /// Ajoute ou met à jour les tags d'un lien existant.
    pub(crate) fn add_tags_to_link(&mut self, from: usize, to: usize, new_tags: Vec<String>) {
        self.tags
            .entry((from, to))
            .or_insert_with(Vec::new)
            .extend(new_tags);
    }

    /// Récupère les tags associés à un lien.
    pub(crate) fn get_tags(&self, from: usize, to: usize) -> Option<&Vec<String>> {
        self.tags.get(&(from, to))
    }

    /// Supprime un tag spécifique d'un lien.
    pub(crate) fn remove_tag(&mut self, from: usize, to: usize, tag: &str) -> bool {
        if let Some(tags) = self.tags.get_mut(&(from, to)) {
            let initial_len = tags.len();
            tags.retain(|t| t != tag);
            return tags.len() < initial_len;
        }
        false
    }

    /// Supprime tous les tags associés à un lien.
    pub(crate) fn clear_tags(&mut self, from: usize, to: usize) {
        self.tags.remove(&(from, to));
    }

    /// Récupère tous les liens associés à un tag spécifique.
    pub(crate) fn get_links_by_tag(&self, tag: &str) -> Vec<(usize, usize)> {
        self.tags
            .iter()
            .filter_map(|(&(from, to), tags)| {
                if tags.contains(&tag.to_string()) {
                    Some((from, to))
                } else {
                    None
                }
            })
            .collect()
    }
}
