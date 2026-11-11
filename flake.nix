{
  description = "Cairn Nix Flake";

  inputs = {
    nixpkgs.url = "https://channels.nixos.org/nixos-unstable/nixexprs.tar.zst";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs: inputs.flake-parts.lib.mkFlake { inherit inputs; } {
    systems = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" ];

    perSystem = { pkgs, ... }: {
      packages.default = pkgs.rustPlatform.buildRustPackage {
        pname = "cairn";
        version = "0.1.0";
        src = ./.;

        cargoLock.lockFile = ./Cargo.lock;

        buildInputs = [
          pkgs.gmp
        ];
      };

      devShells.default = pkgs.mkShell {
        packages = [
          pkgs.cargo
          pkgs.clippy
          pkgs.gmp
          pkgs.rust-analyzer
          pkgs.rustc
          pkgs.rustfmt
        ];
      };
    };
  };
}
