{makeRustPlatform, stdenv, lib, rsTools, makeWrapper, makeBinPath, pandoc}:

let
rustPlatform = makeRustPlatform {
  rustc = rsTools;
  cargo = rsTools;
};
in rustPlatform.buildRustPackage rec {
  pname = "textmachine";
  version = "0.1.0";

  nativeBuildInputs = [ makeWrapper ];

  cargoLock.lockFile = ./Cargo.lock;

  src = lib.cleanSource ./.;

  postInstall = ''
    mkdir -p $out/resources
    cp ./resources/filters.lua $out/resources/filters.lua

    wrapProgram $out/bin/textmachine \
      --prefix PATH : ${makeBinPath [ pandoc ]}
  '';
}
