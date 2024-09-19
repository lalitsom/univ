{
  description = "A devShell for Rust";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-parts.url = "github:hercules-ci/flake-parts";

    # Services flake
    process-compose-flake.url = "github:Platonic-Systems/process-compose-flake";
    services-flake.url = "github:juspay/services-flake";
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "aarch64-darwin" "x86_64-linux" ];

      imports = [
        inputs.process-compose-flake.flakeModule
      ];
      perSystem = { self', pkgs, system, ... }: {
        _module.args.pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [ inputs.rust-overlay.overlays.default ];
        };

        process-compose."services" = {
            imports = [
              inputs.services-flake.processComposeModules.default
            ];

            services.postgres."pg1" = {
              enable = true;
              # initialDatabases = [
              #   {
              #     name = dbName;
              #     schemas = [ "${inputs.northwind}/northwind.sql" ];
              #   }
              # ];
            };
        };

        devShells.default = with pkgs; mkShell {
          buildInputs = [
            openssl
            pkg-config
            eza
            fd
            rust-bin.beta.latest.default
            postgresql
          ];

          shellHook = ''
            alias ls=eza
            alias find=fd
          '';
        };

      };


    };
    #   let
    #     overlays = [ (import rust-overlay) ];
    #     pkgs = import nixpkgs {
    #       inherit system overlays;
    #     };
    #   in
    #   {
    #     devShells.default = with pkgs; mkShell {
    #       buildInputs = [
    #         openssl
    #         pkg-config
    #         eza
    #         fd
    #         rust-bin.beta.latest.default
    #         postgresql
    #       ];

    #       shellHook = ''
    #         alias ls=eza
    #         alias find=fd
    #       '';
    #     };
    #   }
    # );
}