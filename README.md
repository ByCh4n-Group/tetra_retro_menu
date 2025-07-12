# Tetra Menu

Rust + Tetra2D ile shell tarzı oyun menüsü.

## Özellikler

- Shell tarzı retro görünüm
- Türkçe/İngilizce dil desteği
- Animasyonlu cursor ve smooth geçişler
- Modüler kod yapısı

## Kontroller

- **↑/↓**: Menü navigasyon
- **Enter**: Seçim
- **F1/F2**: Dil değiştir (EN/TR)
- **ESC**: Geri/Çıkış

## Kurulum

## Kurulum

```bash
cargo run
```

## Yapı

```
src/
├── main.rs      # Ana oyun döngüsü  
└── language.rs  # Dil sistemi
```

## Teknik

- Tetra2D v0.8
- Serde JSON
