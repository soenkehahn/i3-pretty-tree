{
  inputs.flake-utils.url = "github:numtide/flake-utils";
  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        src = ./.;
      in
      {
        packages = {
          default = pkgs.rustPlatform.buildRustPackage {
            name = "i3-pretty-tree";
            inherit src;
            cargoLock = {
              lockFile = "${src}/Cargo.lock";
              outputHashes = {
                "i3ipc-0.9.0" = "sha256-bL9EnM+LmR1GK1aWRaTAECiYg2+CzdqH7+QzZGX303E=";
              };
            };
          };
        };
        apps = {
          default = flake-utils.lib.mkApp { drv = self.packages.${system}.default; };
        };
      }
    );
}
