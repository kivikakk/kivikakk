{
  description = "kivikakk";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      rec {
        formatter = pkgs.nixfmt-rfc-style;

        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "kvreadme";
          version = "0.1.0";

          src = ./.;

          cargoHash = "sha256-w0HcCzs3YWmj1SKmuSBgryplZtFJ+ANgs0gis/hRw9Y=";
        };

        apps.default = flake-utils.lib.mkApp {
          drv = packages.default;
        };
      }
    );
}
