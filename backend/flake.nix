{
  description = "A Rust project";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  outputs = { self, nixpkgs }: {
    devShell = nixpkgs.lib.mkShell {
      buildInputs = [
          nixpkgs.rustc
          nixpkgs.cargo
          nixpkgs.openssl
          nixpkgs.openssl.dev
          nixpkgs.pkgconfig
          nixpkgs.gcc # This includes the C compiler (cc)
        ];
    };
  };
}