{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [rust-overlay.overlays.default];
        };
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rust-bin.stable.latest.default
            rust-bin.stable.latest.rustfmt
            rust-bin.stable.latest.clippy
            stdenv
          ];
          nativeBuildInputs = [
            pkgs.pkg-config
            pkgs.cmake
            pkgs.vcpkg
          ];
          packages = with pkgs; [
            rust-analyzer
            rustPlatform.bindgenHook
            llvmPackages.libclang.lib
            llvmPackages.clang
            alejandra
          ];
        };
      }
    );
}
