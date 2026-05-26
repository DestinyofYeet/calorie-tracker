{ pkgs, inputs, ... }:
let
  buildWasmBindGenCli =
    pkgs.callPackage "${inputs.nixpkgs}/pkgs/build-support/wasm-bindgen-cli/default.nix"
      { };
in
with pkgs;
[

  (rust-bin.stable.latest.default.override {
    targets = [ "wasm32-unknown-unknown" ];
  })
  (pkgs.callPackage ./wasm-bindgen-cli_0_2_114.nix { inherit buildWasmBindGenCli; })

  dioxus-cli
  rust-analyzer
  rustfmt # formatter
  pkg-config
  wasm-bindgen-cli
  vscode-css-languageserver
  binaryen

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

  # linker
  clang
  wild
]
