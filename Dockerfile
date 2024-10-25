FROM nixos/nix:latest

RUN nix-env -iA nixpkgs.neovim

ENTRYPOINT ["nix", "develop"]
