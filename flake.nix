{
  description = "Rust 6502 emulator development environment";

  inputs = {
    # Stable nixpkgs base + rust-overlay for current stable Rust releases.
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    # Track select fast-moving tools from unstable when needed.
    nixpkgs-unstable.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { nixpkgs, nixpkgs-unstable, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        pkgsUnstable = import nixpkgs-unstable {
          inherit system;
        };

        lib = pkgs.lib;

        # Newer wasm-pack is required than what stable nixpkgs currently provides.
        wasmPackPkg = pkgsUnstable.wasm-pack;

        # Always tracks the latest stable Rust at your current lockfile revision.
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "rustfmt"
            "clippy"
            "rust-analyzer"
          ];
          targets = [ "wasm32-unknown-unknown" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            rustToolchain
            pkg-config
            wasmPackPkg
            binaryen
            python3
          ];

          buildInputs = with pkgs; [
            openssl
            ncurses
          ] ++ lib.optionals stdenv.isDarwin [
            darwin.apple_sdk.frameworks.Security
            darwin.apple_sdk.frameworks.SystemConfiguration
          ];

          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";

          shellHook = ''
            echo "🦀 Rust dev shell ready"
            echo "rustc: $(rustc --version)"
            echo "cargo: $(cargo --version)"
            echo "wasm-pack: $(wasm-pack --version)"
          '';
        };
      }
    );
}
