# This file is pretty general, and you can adapt it in your project replacing
# only `name` and `description` below.
{
  description = "Hyprland-rs dev shell";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    nci.url = "github:yusdacra/nix-cargo-integration";
    nci.inputs.nixpkgs.follows = "nixpkgs";
    parts.url = "github:hercules-ci/flake-parts";
    parts.inputs.nixpkgs-lib.follows = "nixpkgs";
  };

  outputs = inputs @ {
    parts,
    nci,
    ...
  }:
    parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-linux"];
      imports = [nci.flakeModule];
      perSystem = {
        pkgs,
        config,
        ...
      }: let
        crateName = "hyprland";
        crateOutputs = config.nci.outputs.${crateName};
      in {
        # declare projects
        # relPath is the relative path of a project to the flake root
        # TODO: change this to your crate's path
        nci.projects.${crateName}.relPath = "";
        # configure crates
        nci.crates.${crateName} = {
          # export crate (packages and devshell) in flake outputs
          # alternatively you can access the outputs and export them yourself (see below)
          export = true;
          # look at documentation for more options
        };
        # export the crate devshell as the default devshell
        devShells.default = crateOutputs.devShell;
        # export the release package of the crate as default package
        packages.default = crateOutputs.packages.release;
      };
    };
}
