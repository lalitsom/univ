{
  description = "A very basic flake for macOS";

  # Inputs section: Define nixpkgs and rust-overlay
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      forSystem = system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
      in pkgs.mkShell {
        pure = true;
        buildInputs = with pkgs; [
          (pkgs.rust-bin.stable."1.78.0".default)
          pkgs.postgresql_16
          pkgs.diesel-cli
        ] ++ (if pkgs.stdenv.isDarwin then
          [ ]
        else
          []);
        shellHook = ''
          export PS1="(univ-nix-shell) $PS1"
          echo "Entered nix shell for univ rust project .. "
          '';
      };
    in
    {
      devShell.aarch64-darwin = forSystem "aarch64-darwin";
      devShell.aarch64-linux = forSystem "aarch64-linux";
      devShell.x86_64-linux = forSystem "x86_64-linux";
    };
}
