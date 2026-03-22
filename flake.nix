{
  description = "Development shell for my Rust app";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
      in
      {
        nixpkgs.config.allowUnfree = true;

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            pkg-config
            rust-bin.stable.latest.default
          
            libxkbcommon
            wayland
            libX11
            libXcursor
            libXrandr
            libXi
            alsa-lib
            fontconfig
            freetype

            libGL
            vulkan-loader
            vulkan-validation-layers
            mesa

            vulkan-tools
          ];

          shellHook = ''
            export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${
              pkgs.lib.makeLibraryPath [
                pkgs.wayland
                pkgs.libxkbcommon
                pkgs.vulkan-loader
                pkgs.mesa
                pkgs.libGL
              ]
            }"
          '';
        };
      }
    );
}
