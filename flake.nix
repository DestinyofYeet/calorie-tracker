{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";

  };

  outputs =
    { self, ... }@inputs:
    let
      overlays = [ (import inputs.rust-overlay) ];
      pkgs = import inputs.nixpkgs {
        system = "x86_64-linux";
        inherit overlays;
      };

    in
    {
      devShells.x86_64-linux.default = pkgs.mkShell {
        nativeBuildInputs = import ./nix/buildInputs.nix { inherit pkgs inputs; };

        # uncomment this is you get some kind of ssl error, usually on anything networking related using reqwest
        PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";

        # needed env variables
        DATABASE_URL = "./tmp/database.db";
        CSS_ASSET_DIR = "/src/client/assets/css";
      };

      packages.x86_64-linux = {
        default = pkgs.callPackage ./nix/pkg.nix { flake = self; };
      };

      formatter.x86_64-linux =
        let
          script = pkgs.writeShellScriptBin "fmt" ''
            set -euo pipefail

            dx fmt
          '';
        in
        pkgs.symlinkJoin {
          name = "fmt";
          paths = [ script ] ++ (import ./nix/buildInputs.nix { inherit pkgs inputs; });
          buildInputs = [ pkgs.makeWrapper ];
          postBuild = "wrapProgram $out/bin/fmt --prefix PATH : $out/bin";
        };

      checks.x86_64-linux = {
        formatter = pkgs.stdenv.mkDerivation {
          name = "dx-fmt-check";
          src = ./.;
          nativeBuildInputs = import ./nix/buildInputs.nix { inherit pkgs inputs; };
          buildPhase = ''
            dx fmt -c
          '';
        };
      };
    };
}
