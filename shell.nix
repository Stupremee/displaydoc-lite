{ pkgs ? import <nixpkgs> { } }:
let channel = pkgs.rust-bin.stable.latest;
in pkgs.mkShell {
  name = "rust-shell";
  nativeBuildInputs = with pkgs; [
    (channel.rust.override { extensions = [ "rust-src" ]; })
    cargo-expand
  ];
}
