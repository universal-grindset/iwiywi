//! Favorites: user-starred `PulseItem`s persisted to
//! `~/.iwiywi/favorites.json`. Load at startup; `Favorites::toggle` adds
//! or removes an item and rewrites the file.

use std::path::PathBuf;

use crate::pulse::{PulseItem, PulseKind, PulseSource};

pub struct Favorites {
    items: Vec<PulseItem>,
    path: PathBuf,
}

impl Favorites {
    pub fn load_from(path: PathBuf) -> Self {
        let items: Vec<PulseItem> = std::fs::read_to_string(&path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default();
        Self { items, path }
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub fn contains(&self, item: &PulseItem) -> bool {
        self.items
            .iter()
            .any(|i| i.label == item.label && i.body == item.body)
    }

    /// Add the item if not present, remove it if present. Persists the
    /// updated list to disk. Returns `true` if the item is now favorited,
    /// `false` if it was just removed.
    pub fn toggle(&mut self, item: &PulseItem) -> bool {
        if let Some(pos) = self
            .items
            .iter()
            .position(|i| i.label == item.label && i.body == item.body)
        {
            self.items.remove(pos);
            self.save_silent();
            false
        } else {
            // Tag the saved copy as Favorite kind so Focus::Favorites pulls it.
            let saved = PulseItem {
                kind: PulseKind::Favorite,
                step: item.step,
                label: item.label.clone(),
                body: item.body.clone(),
            };
            self.items.push(saved);
            self.save_silent();
            true
        }
    }

    fn save_silent(&self) {
        if let Some(parent) = self.path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Ok(json) = serde_json::to_string_pretty(&self.items) {
            let _ = std::fs::write(&self.path, json);
        }
    }
}

impl PulseSource for Favorites {
    fn name(&self) -> &'static str { "favorites" }
    fn items(&self) -> &[PulseItem] { &self.items }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn sample() -> PulseItem {
        PulseItem {
            kind: PulseKind::BigBookQuote,
            step: Some(3),
            label: "Big Book p. 62".to_string(),
            body: "First of all, we had to quit playing God.".to_string(),
        }
    }

    #[test]
    fn toggle_adds_then_removes() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("favorites.json");
        let mut favs = Favorites::load_from(path.clone());
        let item = sample();
        assert!(!favs.contains(&item));
        assert!(favs.toggle(&item));
        assert!(favs.contains(&item));
        assert!(!favs.toggle(&item));
        assert!(!favs.contains(&item));
    }

    #[test]
    fn saved_item_has_favorite_kind() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("favorites.json");
        let mut favs = Favorites::load_from(path.clone());
        let item = sample();
        favs.toggle(&item);
        assert_eq!(favs.items()[0].kind, PulseKind::Favorite);
        // Step + label + body survive the round-trip.
        assert_eq!(favs.items()[0].step, Some(3));
        assert_eq!(favs.items()[0].label, "Big Book p. 62");
    }

    #[test]
    fn reload_persists_across_instances() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("favorites.json");
        {
            let mut favs = Favorites::load_from(path.clone());
            favs.toggle(&sample());
        }
        let reloaded = Favorites::load_from(path);
        assert_eq!(reloaded.items().len(), 1);
    }

    #[test]
    fn load_from_missing_file_yields_empty() {
        let dir = tempdir().unwrap();
        let favs = Favorites::load_from(dir.path().join("nope.json"));
        assert!(favs.items().is_empty());
    }
}
