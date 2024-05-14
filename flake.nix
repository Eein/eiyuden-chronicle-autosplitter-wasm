{
  inputs = {
    nixpkgs.url = github:nixos/nixpkgs/nixos-unstable;
    flake-utils.url = github:numtide/flake-utils;
    rust-overlay.url = github:oxalica/rust-overlay;
  };

  outputs = { nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachSystem ["x86_64-linux"] (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
      };
    in {
      devShell = pkgs.mkShell {
        buildInputs = with pkgs; [
        ];
        nativeBuildInputs = with pkgs; [
          (rust-bin.nightly.latest.default.override {
            targets = [ "wasm32-unknown-unknown" ];
          })
          openssl
          pkg-config
          wasm-pack
          wasm-bindgen-cli
        ];
      };
    }
  );
}
