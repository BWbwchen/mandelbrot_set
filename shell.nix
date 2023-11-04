let
  pkgs = import (fetchTarball
    # ("https://github.com/NixOS/nixpkgs/archive/621f51253edffa1d6f08d5fce4f08614c852d17e.tar.gz"))
    ("https://github.com/NixOS/nixpkgs/archive/c423f8f656196f9b955f506ce91199e24b180bfa.tar.gz"))
  # ("https://github.com/NixOS/nixpkgs/archive/a58a0b5098f0c2a389ee70eb69422a052982d990.tar.gz"))
    { };

  # Rolling updates, not deterministic.
  # pkgs = import (fetchTarball("channel:nixpkgs-unstable")) {};
in pkgs.mkShell {
  buildInputs = with pkgs; [
    cargo
    rustc
    clippy # for check
    cargo-nextest
  ];
}
