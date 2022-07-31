{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    cargo
    rust-analyzer
    rustc
    rustfmt
  ];
}
