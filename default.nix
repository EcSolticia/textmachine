{makeRustPlatform, stdenv, lib, rsTools}:

let
rustPlatform = makeRustPlatform {
  rustc = rsTools;
  cargo = rsTools;
};
in rustPlatform.buildRustPackage rec {
  pname = "textmachine";
  version = "0.1.0";

  cargoLock.lockFile = ./Cargo.lock;

  src = lib.cleanSource ./.;
}
