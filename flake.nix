{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    ...
  }: (
    builtins.foldl' (acc: elem: nixpkgs.lib.recursiveUpdate acc elem) {} (
      builtins.map (
        system: let
          pkgs = nixpkgs.legacyPackages.${system};
          craneLib = crane.mkLib pkgs;
        in {
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
        }
      )
      [
        "aarch64-darwin"
        "aarch64-linux"
        "i686-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ]
    )
    // {
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
              ExecStart = "${lib.getExe self.packages.${config._module.args.pkgs.stdenv.system}.default} -d";
            };
          };
        };
      };
    }
  );
}
