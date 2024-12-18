{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        packages.default = pkgs.mkShell {
          packages = with pkgs; [
            # tools
            valgrind
            cargo-make
            nodejs_22
            # go
            go
            # c++
            gcc
            # python
            python3
            python3Packages.pip
            maturin
            virtualenv
            libiconv
            # wasm
            wasm-pack
            # rust
            rustup
            gcc
            cmake
            cargo-expand
            rust-cbindgen
          ];
        };

        formatter = pkgs.nixfmt-rfc-style;
      }
    );
}
