{
  inputs = {
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    nixpkgs.url = "nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { self, flake-utils, fenix, nixpkgs, flake-compat, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
            # Add rust nightly to pkgs
        pkgs = nixpkgs.legacyPackages.${system} // { inherit (
          fenix.packages.${system}.latest
        ) rust-src; };
      in rec {
        devShell = pkgs.mkShell {
          packages = with pkgs; [

            (fenix.packages."${system}".latest.withComponents [
              "cargo"
              "clippy"
              "rust-src"
              "rustc"
              "rustfmt"
              "miri"
            ])
          ];
          RUST_SRC_PATH = "${pkgs.rust-src}/lib/rustlib/src/rust/library";
        };

        shellHook = ''
          export DEBUG=1
          cargo build
        '';
      });
}
