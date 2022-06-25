{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-22.05";
    nci.url = "github:yusdacra/nix-cargo-integration";
    nci.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, nci }:
    nci.lib.makeOutputs {
      root = ./.;
      defaultOutputs = {
        app = "rustywatch";
        package = "rustywatch";
      };
    };
}
