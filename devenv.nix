{
  pkgs,
  inputs,
  ...
}: let
  pkgs-unstable = import inputs.nixpkgs-unstable {system = pkgs.stdenv.system;};
in {
  packages = with pkgs; [
    # Code formatting tools
    treefmt
    alejandra
    mdl

    # Rust toolchain
    rustup
    pkgs-unstable.probe-rs
  ];
}
