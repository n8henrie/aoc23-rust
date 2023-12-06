{
  description = "My flake for AoC 2023 in Rust";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/release-23.11";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
  }: let
    inherit (nixpkgs) lib;
    systems = ["aarch64-darwin" "x86_64-linux" "aarch64-linux"];
    systemClosure = attrs:
      builtins.foldl' (acc: system:
        lib.recursiveUpdate acc (attrs system)) {}
      systems;
  in
    systemClosure (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [(import rust-overlay)];
        };
        stable = pkgs.rust-bin.stable.latest.default.override {
          extensions = ["rust-src"];
        };
        nightly = pkgs.rust-bin.selectLatestNightlyWith (toolchain:
          toolchain.default.override {
            extensions = ["rust-src"];
          });
      in {
        packages.${system} = let
          rustPlatform = pkgs.makeRustPlatform {
            rustc = stable;
            cargo = stable;
          };
          workspace = name:
            rustPlatform.buildRustPackage {
              inherit name;
              version = "0.0.1";
              src = ./.;
              cargoLock.lockFile = ./Cargo.lock;
              cargoBuildFlags = "-p ${name}";
            };
          wsDirs = with builtins;
            attrNames (
              lib.filterAttrs (n: v: v == "directory" && match "^d[0-2][0-9]$" n != null)
              (readDir ./.)
            );
        in
          lib.genAttrs wsDirs workspace;

        devShells.${system} = let
          defaultInputs = with pkgs; [
            aoc-cli
            cargo-watch
          ];
        in {
          default = pkgs.mkShell {
            buildInputs = defaultInputs ++ [stable];
            RUST_SRC_PATH = "${stable}/lib/rustlib/src/rust/library";
          };
          nightly = pkgs.mkShell {
            buildInputs = defaultInputs ++ [nightly];
            RUST_SRC_PATH = "${nightly}/lib/rustlib/src/rust/library";
          };
        };
      }
    );
}
