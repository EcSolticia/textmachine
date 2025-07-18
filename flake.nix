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
    in {

      packages.default = pkgs.callPackage ./. {
        inherit rsTools;
      };

      devShells.default = pkgs.mkShell {
        packages = [ rsTools pkgs.bash ];
      };

    });
  
}
