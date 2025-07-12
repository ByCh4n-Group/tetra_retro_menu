mod language;

use tetra::graphics::{self, Color, DrawParams, Texture, TextureFormat};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State, Result, time};

use language::LanguageManager;

#[derive(Debug, Clone, PartialEq)]
pub enum MenuState {
    Main,
    Options,
}

struct GameState {
    language_manager: LanguageManager,
    current_state: MenuState,
    selected_index: usize,
    animation_time: f32,
    fade_alpha: f32,
    game_started: bool,
    main_menu_items: Vec<String>,
    options_menu_items: Vec<String>,
    menu_texture: Texture,
}

impl GameState {
    fn new(ctx: &mut Context) -> Result<GameState> {
        let language_manager = LanguageManager::new();
        
        // 1x1 beyaz texture oluştur - menü öğeleri için  
        let menu_texture = Texture::from_data(ctx, 1, 1, TextureFormat::Rgba8, &[255, 255, 255, 255])?;
        
        let mut state = GameState {
            language_manager,
            current_state: MenuState::Main,
            selected_index: 0,
            animation_time: 0.0,
            fade_alpha: 0.0,
            game_started: false,
            main_menu_items: Vec::new(),
            options_menu_items: Vec::new(),
            menu_texture,
        };
        
        state.update_menu_items();
        Ok(state)
    }
    
    fn update_menu_items(&mut self) {
        self.main_menu_items = vec![
            self.language_manager.get_text("start_game").to_string(),
            self.language_manager.get_text("options").to_string(),
            self.language_manager.get_text("quit").to_string(),
        ];
        
        self.options_menu_items = vec![
            self.language_manager.get_text("volume").to_string(),
            self.language_manager.get_text("graphics").to_string(),
            self.language_manager.get_text("controls").to_string(),
            self.language_manager.get_text("back").to_string(),
        ];
    }
    
    fn get_current_menu_items(&self) -> &Vec<String> {
        match self.current_state {
            MenuState::Main => &self.main_menu_items,
            MenuState::Options => &self.options_menu_items,
        }
    }
    
    fn draw_simple_text(&self, ctx: &mut Context, text: &str, x: f32, y: f32, color: Color) -> Result<()> {
        // Basit blok karakterler ile metin çizimi
        for (i, ch) in text.chars().enumerate() {
            let char_x = x + (i as f32 * 12.0);
            
            // Her karakter için basit pattern
            let blocks = match ch {
                'S' | 's' => vec![(0,0), (1,0), (2,0), (0,1), (1,2), (2,2), (2,3), (0,4), (1,4), (2,4)],
                'T' | 't' => vec![(0,0), (1,0), (2,0), (1,1), (1,2), (1,3), (1,4)],
                'A' | 'a' => vec![(1,0), (0,1), (2,1), (0,2), (1,2), (2,2), (0,3), (2,3), (0,4), (2,4)],
                'R' | 'r' => vec![(0,0), (1,0), (0,1), (2,1), (0,2), (1,2), (0,3), (2,3), (0,4), (2,4)],
                'O' | 'o' => vec![(1,0), (0,1), (2,1), (0,2), (2,2), (0,3), (2,3), (1,4)],
                'P' | 'p' => vec![(0,0), (1,0), (0,1), (2,1), (0,2), (1,2), (0,3), (0,4)],
                'Q' | 'q' => vec![(0,0), (1,0), (2,0), (0,1), (2,1), (0,2), (2,2), (0,3), (1,3), (2,3)],
                'U' | 'u' => vec![(0,0), (2,0), (0,1), (2,1), (0,2), (2,2), (0,3), (2,3), (1,4)],
                'I' | 'i' => vec![(0,0), (1,0), (2,0), (1,1), (1,2), (1,3), (0,4), (1,4), (2,4)],
                'E' | 'e' => vec![(0,0), (1,0), (2,0), (0,1), (0,2), (1,2), (0,3), (0,4), (1,4), (2,4)],
                'N' | 'n' => vec![(0,0), (2,0), (0,1), (1,1), (2,1), (0,2), (2,2), (0,3), (2,3), (0,4), (2,4)],
                'G' | 'g' => vec![(1,0), (2,0), (0,1), (0,2), (1,2), (2,2), (0,3), (2,3), (1,4), (2,4)],
                'M' | 'm' => vec![(0,0), (2,0), (0,1), (1,1), (2,1), (0,2), (2,2), (0,3), (2,3), (0,4), (2,4)],
                'L' | 'l' => vec![(0,0), (0,1), (0,2), (0,3), (0,4), (1,4), (2,4)],
                'C' | 'c' => vec![(1,0), (2,0), (0,1), (0,2), (0,3), (1,4), (2,4)],
                'K' | 'k' => vec![(0,0), (2,0), (0,1), (1,1), (0,2), (0,3), (1,3), (0,4), (2,4)],
                'V' | 'v' => vec![(0,0), (2,0), (0,1), (2,1), (0,2), (2,2), (1,3), (1,4)],
                'B' | 'b' => vec![(0,0), (1,0), (0,1), (2,1), (0,2), (1,2), (0,3), (2,3), (0,4), (1,4)],
                'Y' | 'y' => vec![(0,0), (2,0), (0,1), (2,1), (1,2), (1,3), (1,4)],
                'F' | 'f' => vec![(0,0), (1,0), (2,0), (0,1), (0,2), (1,2), (0,3), (0,4)],
                'D' | 'd' => vec![(0,0), (1,0), (0,1), (2,1), (0,2), (2,2), (0,3), (2,3), (0,4), (1,4)],
                'H' | 'h' => vec![(0,0), (2,0), (0,1), (2,1), (0,2), (1,2), (2,2), (0,3), (2,3), (0,4), (2,4)],
                'Z' | 'z' => vec![(0,0), (1,0), (2,0), (2,1), (1,2), (0,3), (0,4), (1,4), (2,4)],
                'Ş' | 'ş' => vec![(1,0), (2,0), (0,1), (1,2), (2,2), (2,3), (0,4), (1,4), (2,4)],
                'Ğ' | 'ğ' => vec![(1,0), (2,0), (0,1), (0,2), (1,2), (2,2), (0,3), (2,3), (1,4), (2,4)],
                'Ç' | 'ç' => vec![(1,0), (2,0), (0,1), (0,2), (0,3), (1,4), (2,4)],
                'Ö' | 'ö' => vec![(1,0), (0,1), (2,1), (0,2), (2,2), (0,3), (2,3), (1,4)],
                'Ü' | 'ü' => vec![(0,0), (2,0), (0,1), (2,1), (0,2), (2,2), (0,3), (2,3), (1,4)],
                'İ' | 'ı' => vec![(1,0), (1,1), (1,2), (1,3), (1,4)],
                ' ' => vec![],
                _ => vec![(1,2)], // Nokta
            };
            
            for (dx, dy) in blocks {
                let block_x = char_x + (dx as f32 * 3.0);
                let block_y = y + (dy as f32 * 3.0);
                
                self.menu_texture.draw(
                    ctx,
                    DrawParams::new()
                        .position(Vec2::new(block_x, block_y))
                        .scale(Vec2::new(2.0, 2.0))
                        .color(color)
                );
            }
        }
        Ok(())
    }
    
    fn draw_visual_menu(&self, ctx: &mut Context) -> Result<()> {
        // Ana başlık
        self.draw_simple_text(ctx, "TETRA MENU", 250.0, 50.0, 
                             Color::rgba(1.0, 1.0, 0.3, self.fade_alpha))?;
        
        // Menü öğeleri
        let menu_items = self.get_current_menu_items();
        let start_y = 150.0;
        
        for (i, item) in menu_items.iter().enumerate() {
            let y = start_y + (i as f32 * 50.0);
            let x = 200.0;
            
            // Seçili öğe için cursor
            if i == self.selected_index {
                self.draw_simple_text(ctx, "> ", x - 50.0, y, 
                                     Color::rgba(1.0, 1.0, 0.3, self.fade_alpha))?;
                
                // Highlight arka plan
                for highlight_x in (x as i32 - 20..x as i32 + 300).step_by(8) {
                    for highlight_y in (y as i32..y as i32 + 30).step_by(4) {
                        self.menu_texture.draw(
                            ctx,
                            DrawParams::new()
                                .position(Vec2::new(highlight_x as f32, highlight_y as f32))
                                .scale(Vec2::new(6.0, 2.0))
                                .color(Color::rgba(0.2, 0.4, 0.8, self.fade_alpha * 0.2))
                        );
                    }
                }
            }
            
            // Menü item metni
            let item_color = if i == self.selected_index {
                Color::rgba(1.0, 1.0, 0.3, self.fade_alpha)
            } else {
                Color::rgba(0.8, 0.8, 0.8, self.fade_alpha * 0.7)
            };
            
            self.draw_simple_text(ctx, item, x, y, item_color)?;
        }
        
        // Alt bilgi
        let window_size = tetra::window::get_size(ctx);
        let footer_y = window_size.1 as f32 - 80.0;
        
        self.draw_simple_text(ctx, "F1: EN  F2: TR", 50.0, footer_y, 
                             Color::rgba(0.6, 0.6, 0.6, self.fade_alpha))?;
        
        self.draw_simple_text(ctx, "ENTER: SELECT  ESC: BACK", 300.0, footer_y,
                             Color::rgba(0.6, 0.6, 0.6, self.fade_alpha))?;
        
        Ok(())
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> Result {
        let dt = time::get_delta_time(ctx).as_secs_f32();
        
        // Animasyon güncelleme
        self.animation_time += dt;
        self.fade_alpha = (self.animation_time * 2.0).min(1.0);
        
        // Konsol çıktısını azalt
        if self.animation_time.fract() < 0.1 {
            let current_menu = match self.current_state {
                MenuState::Main => "Main",
                MenuState::Options => "Options",
            };
            
            println!("🎮 Menu: {} | Selected: {} | Item: {}", 
                     current_menu, self.selected_index, 
                     self.get_current_menu_items().get(self.selected_index).unwrap_or(&"None".to_string()));
        }
        
        // Dil değiştirme
        if input::is_key_pressed(ctx, Key::F1) {
            self.language_manager.set_language("en");
            self.update_menu_items();
            println!("🌍 Language changed to English");
        } else if input::is_key_pressed(ctx, Key::F2) {
            self.language_manager.set_language("tr");
            self.update_menu_items();
            println!("🌍 Dil Türkçe olarak değiştirildi");
        }
        
        if self.game_started {
            if input::is_key_pressed(ctx, Key::Escape) {
                self.game_started = false;
                self.current_state = MenuState::Main;
                self.selected_index = 0;
                self.animation_time = 0.0;
                println!("⬅️ Menüye dönüldü / Returned to menu");
            }
            return Ok(());
        }
        
        // Menü navigasyonu
        let menu_len = self.get_current_menu_items().len();
        
        if input::is_key_pressed(ctx, Key::Up) {
            if self.selected_index > 0 {
                self.selected_index -= 1;
            } else {
                self.selected_index = menu_len - 1;
            }
            println!("⬆️ Selected: {} - {}", self.selected_index, 
                     self.get_current_menu_items()[self.selected_index]);
        }
        
        if input::is_key_pressed(ctx, Key::Down) {
            if self.selected_index < menu_len - 1 {
                self.selected_index += 1;
            } else {
                self.selected_index = 0;
            }
            println!("⬇️ Selected: {} - {}", self.selected_index, 
                     self.get_current_menu_items()[self.selected_index]);
        }
        
        if input::is_key_pressed(ctx, Key::Enter) {
            match self.current_state {
                MenuState::Main => {
                    match self.selected_index {
                        0 => {
                            self.game_started = true;
                            println!("🎮 Oyun başlatılıyor! / Game starting!");
                        }
                        1 => {
                            self.current_state = MenuState::Options;
                            self.selected_index = 0;
                            println!("⚙️ Ayarlar menüsüne girildi / Entered options menu");
                        }
                        2 => {
                            println!("🚪 Çıkış yapılıyor... / Exiting...");
                            tetra::window::quit(ctx);
                        }
                        _ => {}
                    }
                }
                MenuState::Options => {
                    match self.selected_index {
                        0 => println!("🔊 Ses ayarları / Volume settings"),
                        1 => println!("🎨 Grafik ayarları / Graphics settings"),
                        2 => println!("🎮 Kontrol ayarları / Control settings"),
                        3 => {
                            self.current_state = MenuState::Main;
                            self.selected_index = 0;
                            println!("⬅️ Ana menüye dönüldü / Back to main menu");
                        }
                        _ => {}
                    }
                }
            }
        }
        
        if input::is_key_pressed(ctx, Key::Backspace) {
            if self.current_state == MenuState::Options {
                self.current_state = MenuState::Main;
                self.selected_index = 0;
                println!("⬅️ Ana menüye dönüldü / Back to main menu");
            }
        }
        
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result {
        if self.game_started {
            // Oyun ekranı - parlak yeşil
            graphics::clear(ctx, Color::rgb(0.1, 0.6, 0.1));
            
            // "OYUN EKRANI" yazısı
            self.draw_simple_text(ctx, "OYUN EKRANI", 300.0, 250.0, Color::WHITE)?;
            self.draw_simple_text(ctx, "GAME SCREEN", 300.0, 300.0, Color::WHITE)?;
            self.draw_simple_text(ctx, "ESC: MENU", 320.0, 400.0, Color::rgba(1.0, 1.0, 0.3, 0.8))?;
        } else {
            // Menü ekranı - animasyonlu gradient
            let fade = self.fade_alpha;
            let time_wave = (self.animation_time * 0.5).sin() * 0.05;
            
            match self.current_state {
                MenuState::Main => {
                    // Ana menü - mavi tonları
                    graphics::clear(ctx, Color::rgb(
                        0.02 + time_wave, 
                        0.05 + time_wave, 
                        0.15 * fade + time_wave
                    ));
                }
                MenuState::Options => {
                    // Ayarlar menüsü - mor tonları  
                    graphics::clear(ctx, Color::rgb(
                        0.10 * fade + time_wave,
                        0.02 + time_wave,
                        0.15 * fade + time_wave
                    ));
                }
            }
            
            // Görsel menü çizimi
            self.draw_visual_menu(ctx)?;
        }
        
        Ok(())
    }
}

fn main() -> Result {
    println!("🚀 Tetra2D Oyun Menüsü başlatılıyor...");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🎮 KONTROLLER:");
    println!("   ↑/↓     : Menüde hareket");
    println!("   Enter   : Seçim yapma");
    println!("   F1      : İngilizce");
    println!("   F2      : Türkçe");
    println!("   Backspace: Geri (ayarlarda)");
    println!("   ESC     : Oyundan menüye dön");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    ContextBuilder::new("Tetra Game Menu", 800, 600)
        .quit_on_escape(false)
        .build()?
        .run(GameState::new)
}
