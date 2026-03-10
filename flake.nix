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

      buildWasmBindGenCli =
        pkgs.callPackage "${inputs.nixpkgs}/pkgs/build-support/wasm-bindgen-cli/default.nix"
          { };
    in
    {
      devShells.x86_64-linux.default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          (rust-bin.stable.latest.default.override {
            targets = [ "wasm32-unknown-unknown" ];
          })
          (pkgs.callPackage ./nix/wasm-bindgen-cli_0_2_114.nix { inherit buildWasmBindGenCli; })

          dioxus-cli
          rust-analyzer
          rustfmt # formatter
          pkg-config
          wasm-bindgen-cli

          # diesel-cli
          diesel-cli
          libpq
          libmysqlclient
          sqlite.dev

          # lsp
          pango
          atk
          libsoup_3
          gdk-pixbuf
          webkitgtk_4_1

          # linux desktop
          xdotool
        ];

        # uncomment this is you get some kind of ssl error, usually on anything networking related using reqwest
        PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
        DATABASE_URL = "sqlite://tmp/database.db";
      };

      packages.x86_64-linux = {
        default = pkgs.callPackage ./nix/pkg.nix { };
      };
    };
}
