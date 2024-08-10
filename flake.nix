{
  description = "Flake utils demo";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        riscvPkgs = import nixpkgs {
          crossSystem = {
            config = "riscv64-unknown-linux-gnu";
          };
          inherit system;
        };
      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
            cargo-binutils
            qemu
            gdb
            riscvPkgs.buildPackages.gcc
          ];

          shellHook = ''
            export CC=riscv64-unknown-linux-gnu-gcc
          '';
        };
      }
    );
}
