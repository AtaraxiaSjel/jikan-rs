{
  description = "devenv for jikan-rs lib";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    devenv.url = "github:cachix/devenv";
    fenix = {
      url = "github:nix-community/fenix/monthly";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  nixConfig = {
    extra-trusted-public-keys = "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
    extra-substituters = "https://devenv.cachix.org";
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ inputs.devenv.flakeModule ];
      systems = [ "x86_64-linux" ];

      perSystem = { config, self', inputs', pkgs, system, lib, ... }:
        {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [ inputs.fenix.overlays.default ];
          };
          devenv.shells.default = let
            libs = with pkgs; [ openssl ];
            toolchain = (pkgs.fenix.complete.withComponents [
              "cargo"
              "clippy"
              "rust-docs"
              "rust-src"
              "rustc"
              "rustfmt"
            ]);
          in {
            name = "jikan-rs-devenv";
            env = {
              LD_LIBRARY_PATH = lib.makeLibraryPath libs;
              LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
            };
            packages = with pkgs; libs ++ [
              nixfmt-rfc-style
              rust-analyzer-nightly
              toolchain
            ];
            languages.rust = {
              enable = true;
              channel = "nightly";
              components = [];
              toolchain = lib.mkForce toolchain;
            };
          };
        };
    };
}
