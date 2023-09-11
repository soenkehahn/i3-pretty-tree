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
                "swayipc-3.0.1" = "sha256-/ktPyC2OSPSR8iplpeIHKfwdrmaWH11c6VoZkm3uQzY=";
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
