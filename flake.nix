{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    crane,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystemPassThrough (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
        craneLib = crane.mkLib pkgs;
      in rec {
        packages.${system} = rec {
          hayabusa = craneLib.buildPackage {
            src = ./.;
            nativeBuildInputs = with pkgs; [
              pkg-config
              openssl
            ];
            meta = {
              homepage = "https://github.com/Notarin/hayabusa/";
              description = "Hayabusa is a swift rust fetch program.";
              mainProgram = "hayabusa";
            };
          };
          default = hayabusa;
        };
        devShells.${system}.default = craneLib.devShell {
          packages = with pkgs; [
            pkg-config
            openssl
          ];
        };
        nixosModules.default = {
          pkgs,
          lib,
          config,
          ...
        }: {
          options = {
            services.hayabusa = {
              enable = lib.mkEnableOption "Enable the hayabusa system info daemon";
            };
          };
          config = lib.mkIf config.services.hayabusa.enable {
            systemd.services.hayabusa = {
              after = ["network.target"];
              wants = ["network-online.target"];
              wantedBy = ["multi-user.target"];
              serviceConfig = {
                Restart = "always";
                Type = "simple";
                ExecStart = "${lib.getExe packages.${config._module.args.pkgs.stdenv.system}.default} -d";
              };
            };
          };
        };
      }
    );
}
