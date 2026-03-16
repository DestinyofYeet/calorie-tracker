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
        DATABASE_URL = "sqlite://tmp/database.db";
        CSS_ASSET_DIR = "/src/client/assets/css";
      };

      packages.x86_64-linux = {
        default = pkgs.callPackage ./nix/pkg.nix { flake = self; };
      };
    };
}
