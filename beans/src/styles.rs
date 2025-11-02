//! Centralized styling constants for Freya UI
//!
//! This module replaces the CSS file with inline styling constants
//! that can be used throughout the Freya-based UI.

/// Color constants
pub mod colors {
    pub const BACKGROUND: &str = "#f5f5f5";
    pub const RIBBON_BG: &str = "#ffffff";
    pub const RIBBON_BORDER: &str = "#ccc";
    
    pub const BUTTON_BG: &str = "#f0f0f0";
    pub const BUTTON_HOVER: &str = "#e8e8e8";
    pub const BUTTON_BORDER: &str = "#bbb";
    pub const BUTTON_DISABLED: &str = "#e0e0e0";
    
    pub const TEXT_PRIMARY: &str = "#333";
    pub const TEXT_SECONDARY: &str = "#666";
    pub const TEXT_DISABLED: &str = "#999";
    
    pub const SUCCESS: &str = "#4caf50";
    pub const ERROR: &str = "#f44336";
    pub const WARNING: &str = "#ff9800";
    pub const INFO: &str = "#2196f3";
    
    pub const INCOME: &str = "#4caf50";
    pub const EXPENSE: &str = "#f44336";
    
    pub const BORDER: &str = "#ddd";
    pub const SHADOW: &str = "rgba(0, 0, 0, 0.1)";
}

/// Spacing constants (in pixels)
pub mod spacing {
    pub const TINY: &str = "4";
    pub const SMALL: &str = "8";
    pub const MEDIUM: &str = "12";
    pub const LARGE: &str = "16";
    pub const XLARGE: &str = "24";
}

/// Font sizes
pub mod fonts {
    pub const SMALL: &str = "12";
    pub const NORMAL: &str = "14";
    pub const MEDIUM: &str = "16";
    pub const LARGE: &str = "18";
    pub const XLARGE: &str = "24";
    pub const TITLE: &str = "32";
}

/// Border radius
pub mod radius {
    pub const SMALL: &str = "4";
    pub const MEDIUM: &str = "8";
    pub const LARGE: &str = "12";
}

