{
  description = "Beans - A multi-platform ledger application built with Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        # Use stable Rust with specific extensions
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" "rustfmt" ];
          targets = [ "x86_64-unknown-linux-gnu" "x86_64-apple-darwin" "aarch64-apple-darwin" ];
        };

        # Native build inputs for all platforms
        nativeBuildInputs = with pkgs; [
          rustToolchain
          pkg-config
          cmake
          clang
        ];

        # Build inputs specific to each platform
        buildInputs = with pkgs; [
          # SQLite dependencies
          sqlite
          
          # OpenSSL for reqwest
          openssl
          
          # Skia dependencies for Freya
          fontconfig
          xorg.libX11
          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
          
          # GTK and WebKit dependencies for Dioxus Desktop
          gtk3
          webkitgtk
          libsoup
          xdotool
          
          # Common dependencies
          libGL
          vulkan-loader
          
          # Platform-specific libraries
        ] ++ pkgs.lib.optionals pkgs.stdenv.isLinux [
          # Linux-specific dependencies
          libxkbcommon
          wayland
        ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
          # macOS-specific dependencies
          pkgs.darwin.apple_sdk.frameworks.Security
          pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
          pkgs.darwin.apple_sdk.frameworks.AppKit
          pkgs.darwin.apple_sdk.frameworks.WebKit
          pkgs.darwin.apple_sdk.frameworks.CoreFoundation
          pkgs.darwin.apple_sdk.frameworks.CoreServices
          pkgs.darwin.apple_sdk.frameworks.CoreGraphics
          pkgs.darwin.apple_sdk.frameworks.Foundation
          pkgs.libiconv
        ];

        # Development shell with additional tools
        devShell = pkgs.mkShell {
          inherit nativeBuildInputs buildInputs;
          
          # Additional development tools
          packages = with pkgs; [
            cargo-watch    # For auto-reloading during development
            cargo-audit    # For security audits
            cargo-tarpaulin # For code coverage
            cargo-expand   # For macro expansion
            sqlitebrowser # For viewing SQLite databases
          ];
          
          # Environment variables
          shellHook = ''
            echo "🫘 Welcome to the Beans development environment! 🫘"
            echo "Rust toolchain: $(rustc --version)"
            echo "Cargo: $(cargo --version)"
            echo ""
            echo "Available commands:"
            echo "  cargo build        - Build the project"
            echo "  cargo test         - Run tests"
            echo "  cargo doc --open   - Generate and view documentation"
            echo "  cargo watch -x run - Run with auto-reload on changes"
            echo ""
            
            # Fix for black screen issue with Dioxus Desktop on Linux
            export WEBKIT_DISABLE_COMPOSITING_MODE=1
            
            # For Skia/Freya GPU acceleration
            export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath [
              pkgs.libGL
              pkgs.vulkan-loader
            ]}:$LD_LIBRARY_PATH
          '';
        };
      in
      {
        # Development shell
        devShells.default = devShell;
        
        # For `nix build` - builds the binary package
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "beans";
          version = "0.1.0";
          src = ./.;
          
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
          
          nativeBuildInputs = nativeBuildInputs;
          buildInputs = buildInputs;
        };
      }
    );
}
