# NOTE: nixos/nix:latest だと wasm-bindgen が動作しないので代わりに ubuntu:22.04 を使用
FROM ubuntu:22.04

RUN apt-get update && apt-get install -y curl xz-utils && apt-get clean 

# install nix
RUN curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix | sh -s -- install linux --init none --no-confirm
RUN echo ". /root/.nix-profile/etc/profile.d/nix.sh" >> /root/.bashrc
