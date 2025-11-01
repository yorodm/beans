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
        
        # Use nightly Rust with specific extensions (required for Ribir)
        rustToolchain = pkgs.rust-bin.nightly.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" "rustfmt" ];
          targets = [ "x86_64-unknown-linux-gnu" "x86_64-apple-darwin" "aarch64-apple-darwin" ];
        };

        # Native build inputs for all platforms
        nativeBuildInputs = with pkgs; [
          rustToolchain
          pkg-config
        ];

        # Build inputs specific to each platform
        buildInputs = with pkgs; [
          # SQLite dependencies
          sqlite
          
          # OpenSSL for reqwest
          openssl
          
          # Platform-specific libraries
        ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
          # macOS-specific dependencies
          pkgs.darwin.apple_sdk.frameworks.Security
          pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
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
