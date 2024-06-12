{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/ae34cb9560a578b6354655538e98fb69e8bc8d39";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ ];
        };
      in
      {
        devShell =
          with pkgs;
          with pkgs.darwin.apple_sdk.frameworks;
          mkShell {
            buildInputs = [
              rustup
              libiconv
              cargo-watch
            ];
          };
      });
}
