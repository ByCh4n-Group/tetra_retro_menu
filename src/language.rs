use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageData {
    pub menu_title: String,
    pub start_game: String,
    pub options: String,
    pub quit: String,
    pub language: String,
    pub back: String,
    pub volume: String,
    pub graphics: String,
    pub controls: String,
    pub press_key: String,
}

pub struct LanguageManager {
    current_language: String,
    languages: HashMap<String, LanguageData>,
}

impl LanguageManager {
    pub fn new() -> Self {
        let mut languages = HashMap::new();

        // English translations
        languages.insert(
            "en".to_string(),
            LanguageData {
                menu_title: "MAIN MENU".to_string(),
                start_game: "🎮 Start Game".to_string(),
                options: "⚙️ Options".to_string(),
                quit: "🚪 Quit".to_string(),
                language: "🌍 Language".to_string(),
                back: "⬅️ Back".to_string(),
                volume: "🔊 Volume Settings".to_string(),
                graphics: "🎨 Graphics Settings".to_string(),
                controls: "🎮 Control Settings".to_string(),
                press_key: "Press any key...".to_string(),
            },
        );

        // Turkish translations
        languages.insert(
            "tr".to_string(),
            LanguageData {
                menu_title: "ANA MENÜ".to_string(),
                start_game: "🎮 Oyunu Başlat".to_string(),
                options: "⚙️ Ayarlar".to_string(),
                quit: "🚪 Çıkış".to_string(),
                language: "🌍 Dil".to_string(),
                back: "⬅️ Geri".to_string(),
                volume: "🔊 Ses Ayarları".to_string(),
                graphics: "🎨 Grafik Ayarları".to_string(),
                controls: "🎮 Kontrol Ayarları".to_string(),
                press_key: "Herhangi bir tuşa basın...".to_string(),
            },
        );

        Self {
            current_language: "en".to_string(),
            languages,
        }
    }

    pub fn set_language(&mut self, language: &str) {
        if self.languages.contains_key(language) {
            self.current_language = language.to_string();
        }
    }

    pub fn get_current_language(&self) -> &str {
        &self.current_language
    }

    pub fn get_text(&self, key: &str) -> &str {
        if let Some(lang_data) = self.languages.get(&self.current_language) {
            match key {
                "menu_title" => &lang_data.menu_title,
                "start_game" => &lang_data.start_game,
                "options" => &lang_data.options,
                "quit" => &lang_data.quit,
                "language" => &lang_data.language,
                "back" => &lang_data.back,
                "volume" => &lang_data.volume,
                "graphics" => &lang_data.graphics,
                "controls" => &lang_data.controls,
                "press_key" => &lang_data.press_key,
                _ => "Unknown",
            }
        } else {
            "Unknown"
        }
    }

    pub fn get_language_data(&self) -> Option<&LanguageData> {
        self.languages.get(&self.current_language)
    }
}
