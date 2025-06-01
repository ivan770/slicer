{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    nix-filter.url = "github:numtide/nix-filter";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      nix-filter,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        src = nix-filter.lib.filter {
          root = ./.;

          include = [
            "src"
            "Cargo.toml"
            "Cargo.lock"
          ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustc
            cargo
            clippy
            rustfmt
          ];
        };

        packages.default = pkgs.callPackage ./nix/package.nix {
          inherit src;
        };

        formatter = pkgs.nixfmt-tree;
      }
    );
}
