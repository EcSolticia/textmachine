{
  description = "textmachine flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-25.05";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system: let
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs {
        inherit system overlays;
      };
      rsTools = pkgs.rust-bin.stable."1.88.0".default;
      pandoc = pkgs.pandoc;
    in {

      packages.default = pkgs.callPackage ./. {
        inherit rsTools;
        inherit pandoc;
        makeWrapper = pkgs.makeWrapper;
        makeBinPath = pkgs.lib.makeBinPath;
      };

      devShells.default = pkgs.mkShell {
        packages = [ rsTools pandoc pkgs.bash ];
      };

    });
  
}
