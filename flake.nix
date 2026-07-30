{
  description = "pretend";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    inputs@{
      self,
      nixpkgs,
      flake-parts,
      treefmt-nix,
      rust-overlay,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
      ];

      perSystem =
        {
          system,
          pkgs,
          ...
        }:
        let
          overlays = [
            rust-overlay.overlays.default
          ];

          pkgs' = import nixpkgs {
            inherit system;
            overlays = overlays;
          };

          rustToolchain = pkgs'.rust-bin.stable.latest.default.override {
            extensions = [
              "rust-src"
              "rust-analyzer"
              "clippy"
              "rustfmt"
            ];
          };
        in
        {
          formatter = import ./.nix/formatter.nix {
            pkgs = pkgs';
            inputs = inputs;
          };

          devShells.default = pkgs'.mkShell {
            packages = [
              rustToolchain
              pkgs'.cargo
              pkgs'.pkg-config
              pkgs'.openssl
            ];

            shellHook = ''
              export RUST_SRC_PATH="${rustToolchain}/lib/rustlib/src/rust/library"
            '';
          };
        };
    };
}
