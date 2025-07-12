<!-- Use this file to provide workspace-specific custom instructions to Copilot. For more details, visit https://code.visualstudio.com/docs/copilot/copilot-customization#_use-a-githubcopilotinstructionsmd-file -->

# Tetra2D Menu System - Copilot Instructions

This is a Rust project using the Tetra2D game engine to create a modular shell-style menu system.

## Project Context

- **Language**: Rust
- **Framework**: Tetra2D (2D game engine)
- **Architecture**: Modular design with separate modules for menu, language, and cursor systems
- **Style**: Shell/terminal-inspired UI with animated cursor and smooth transitions

## Key Design Principles

1. **Modularity**: Each system (menu, language, cursor) is in its own module
2. **Internationalization**: Multi-language support with easy translation management
3. **Smooth Animation**: All UI elements should have smooth, pleasant animations
4. **Shell Aesthetic**: Terminal-like appearance with blinking cursor and clean text

## Code Style Guidelines

- Use descriptive variable and function names
- Implement proper error handling with `tetra::Result`
- Keep modules focused on single responsibilities
- Use enums for menu actions and states
- Implement smooth animations using delta time

## Module Responsibilities

- `menu.rs`: Menu navigation, rendering, and state management
- `language.rs`: Translation management and language switching
- `cursor.rs`: Animated cursor with blinking and smooth movement
- `main.rs`: Application entry point and main game loop

## When adding new features:

1. Consider the modular architecture
2. Add appropriate translations for new text
3. Implement smooth animations where applicable
4. Follow the existing error handling patterns
5. Maintain the shell-style aesthetic
