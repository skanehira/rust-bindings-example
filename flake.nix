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
            cargo-make
            nodejs_22
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
          ];
        };

        formatter = pkgs.nixfmt-rfc-style;
      }
    );
}
