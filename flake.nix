{
  description = "textmachine flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-25.05";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay, ... }: let
    systems = ["x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin"];
    forAllSystems = nixpkgs.lib.genAttrs systems;
  in {

    packages = forAllSystems (system: let
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs {
        inherit system overlays;
      };
      rsTools = pkgs.rust-bin.stable."1.88.0".default;
      textmachinePkg = pkgs.callPackage ./. {
        inherit rsTools;
      };
    in {
      default = textmachinePkg;
    });

    devShells = forAllSystems (system: let
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs {
        inherit system overlays;
      };
      rsTools = pkgs.rust-bin.stable."1.88.0".default;
    in {
      default = pkgs.mkShell {
        packages = [rsTools pkgs.bash];
      };
    });
    
  };
}
