{ sources ? import ./nix/sources.nix }:
let
  pkgs = import sources.nixpkgs { };
in
pkgs.mkShell
{
  buildInputs = [
    # Rust
    pkgs.rustup
    pkgs.libiconv
  ];


  shellHook = ''
  '';
}
