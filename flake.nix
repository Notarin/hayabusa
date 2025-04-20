{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      crane,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        craneLib = crane.mkLib pkgs;
      in
      rec {
        packages.hayabusa = craneLib.buildPackage {
          src = ./.;
          nativeBuildInputs = with pkgs; [
            pkg-config
            openssl
          ];
        };
        packages.default = packages.hayabusa;
        devShells.default = craneLib.devShell {
          packages = with pkgs; [
            pkg-config
            openssl
          ];
        };
      }
    );
}
