{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    nixpkgs-unstable.url = "github:NixOS/nixpkgs/nixos-unstable";

    systems.url = "github:nix-systems/default";
  };

  outputs = { systems, nixpkgs, nixpkgs-unstable, ... }:
    let
      eachSystem = f:
        nixpkgs.lib.genAttrs (import systems) (system:
          let
            overlay-unstable = final: prev: {
              unstable = nixpkgs-unstable.legacyPackages.${prev.system};
            };
          in f (import nixpkgs {
            inherit system;
            overlays = [ overlay-unstable ];
          }));
    in {
      devShell = eachSystem (pkgs:
        pkgs.mkShell {
          packages = with pkgs; [
            cargo
            rustc
            rust-analyzer
            clippy
            rustfmt
            pkg-config
            openssl
          ];
        });
    };
}
