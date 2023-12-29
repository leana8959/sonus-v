{
  description = "sonus v";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = import nixpkgs { inherit system; };
      in {

        devShell = pkgs.mkShell {
          packages = with pkgs; [
            cmake
            pkg-config
            fontconfig
            hello
            xorg.libX11
            xorg.libXrandr
            xorg.libXinerama
            xorg.libXcursor
            xorg.libXi
            raylib
          ];
          PKG_CONFIG = pkgs.pkg-config;
        };

      });
}
