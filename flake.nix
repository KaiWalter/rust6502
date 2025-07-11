{
  description = "A Rust 6502 emulator development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        # Use the latest stable Rust toolchain with additional components
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "clippy" "rustfmt" ];
          targets = [ "wasm32-unknown-unknown" ];
        };
        
        # Native dependencies for the project
        nativeBuildInputs = with pkgs; [
          rustToolchain
          pkg-config
          # For WASM development
          wasm-pack
          # For serving WASM applications during development
          python3
        ];
        
        # Runtime dependencies
        buildInputs = with pkgs; [
          # ncurses for the apple1 terminal interface
          ncurses
          # OpenSSL for potential network dependencies
          openssl
        ] ++ lib.optionals stdenv.isDarwin [
          # macOS specific dependencies
          darwin.apple_sdk.frameworks.Security
          darwin.apple_sdk.frameworks.SystemConfiguration
        ];

      in
      {
        devShells.default = pkgs.mkShell {
          inherit buildInputs nativeBuildInputs;
          
          # Environment variables
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          
          # Shell hook to display helpful information
          shellHook = ''
            echo "🦀 Rust 6502 Development Environment"
            echo "======================================"
            echo ""
            echo "Available commands:"
            echo "  cargo build          - Build the project"
            echo "  cargo test           - Run tests"
            echo "  cargo run            - Run the functional test"
            echo "  cargo run --bin apple1 - Run the Apple 1 emulator"
            echo ""
            echo "WASM development:"
            echo "  wasm-pack build apple1-wasm --target web"
            echo "  python3 -m http.server 8000  # Serve WASM files"
            echo ""
            echo "Rust version: $(rustc --version)"
            echo "Cargo version: $(cargo --version)"
            echo ""
          '';
        };
      });
}
