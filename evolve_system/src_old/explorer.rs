/// Ce fichier définit la structure `Explorer` et ses méthodes associées.
/// Rôle : Appliquer des transformations ou explorer des combinaisons sans gérer directement des stratégies.
use rand::seq::SliceRandom;

pub struct Explorer;

impl Explorer {
    pub fn new() -> Self {
        Self
    }

    /// Applique une transformation aléatoire sur une entrée à partir d'une liste de fonctions
    pub fn try_random_action(
        &self,
        input: &str,
        transformations: &[Box<dyn Fn(&str) -> String + Send + Sync>],
    ) -> String {
        let mut rng = rand::rng();
        let strat = transformations.choose(&mut rng).unwrap();
        strat(input)
    }

    /// Applique toutes les transformations sur une entrée
    pub fn try_all(
        &self,
        input: &str,
        transformations: &[Box<dyn Fn(&str) -> String + Send + Sync>],
    ) -> Vec<String> {
        transformations.iter().map(|strat| strat(input)).collect()
    }

    /// Crée une nouvelle transformation composée de deux transformations existantes
    pub fn meta_explore<'a>(
        &self,
        transformations: &'a [Box<dyn Fn(&str) -> String + Send + Sync>],
    ) -> Option<Box<dyn Fn(&str) -> String + Send + Sync + 'a>> {
        let mut rng = rand::rng();

        if transformations.len() < 2 {
            return None;
        }

        let (f1, f2) = {
            let mut choices = transformations.choose_multiple(&mut rng, 2);
            (choices.next().unwrap(), choices.next().unwrap())
        };

        Some(Box::new(move |input| {
            let mid = f1(input);
            f2(&mid)
        }))
    }
}
