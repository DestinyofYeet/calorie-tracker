{
  rustPlatform,
  lib,
  pkgs,
  flake,
  ...
}:

rustPlatform.buildRustPackage rec {
  pname = "calorie-tracker";
  version = "1.0";

  nativeBuildInputs = import ./buildInputs.nix {
    inherit pkgs;
    inputs = flake.inputs;
  };

  src = ../.;

  cargoHash = "sha256-JtOQa7O3R0bj/CC1ddA4VgCAvfdwbBao0aQR6BzYlFI=";

  buildPhase = ''
    dx build -r --web
  '';

  installPhase = ''
    mkdir -p $out/bin
    cp -r target/dx/${pname}/release/web/* $out/bin
  '';

  meta = with lib; {
    mainProgram = pname;
    description = "A program";
    license = licenses.agpl3Only;
    platforms = platforms.all;
  };
}
