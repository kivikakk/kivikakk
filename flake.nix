{
  description = "kivikakk";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
  };

  outputs =
    {
      self,
      nixpkgs,
    }:
    let
      systems = [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];
      eachSystem = nixpkgs.lib.genAttrs systems;
    in
    rec {
      formatter = eachSystem (system: nixpkgs.legacyPackages.${system}.nixfmt-rfc-style);

      packages = eachSystem (system: rec {
        default = kvreadme;

        kvreadme = nixpkgs.legacyPackages.${system}.rustPlatform.buildRustPackage {
          pname = "kvreadme";
          version = "0.1.0";

          src = ./.;

          cargoHash = "sha256-w0HcCzs3YWmj1SKmuSBgryplZtFJ+ANgs0gis/hRw9Y=";
        };
      });

      devShells = packages;
    };
}
