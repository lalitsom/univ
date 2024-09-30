{
  description = "A very basic flake for macOS";

  # Inputs section: Define nixpkgs and rust-overlay
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-23.05-darwin";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  # Outputs section: Define the devShell
  outputs = { self, nixpkgs, rust-overlay }: {
    # Define the devShell for aarch64-darwin (macOS)
    devShell.aarch64-darwin = let
      # Import nixpkgs and apply the rust-overlay
      pkgs = import nixpkgs {
        system = "aarch64-darwin";
        overlays = [ (import rust-overlay) ];  # Apply the rust-overlay
      };
    in pkgs.mkShell {
      pure = true;
      buildInputs = with pkgs; [
        (pkgs.rust-bin.stable."1.78.0".default)
      ];

      
      shellHook = ''
        echo "Entered nix shell for univ rust project..."
      '';
    };
  };
}
