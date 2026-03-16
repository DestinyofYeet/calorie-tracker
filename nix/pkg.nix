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

  cargoHash = "sha256-mt/syi+G/j18qVB8nJRsOdbeLbNN95tGTLYGIuzQ1Uo=";

  buildPhase = ''
    dx bundle -r --web
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
