{
  description = "Screenland is a program for creating and editing screenshots";

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
        buildInputs = with pkgs; [
            libxkbcommon
            wayland
            libX11
            libXcursor
            libXrandr
            libXi
            libGL
            vulkan-loader
            vulkan-validation-layers
            mesa

            vulkan-tools
          ];
      in
      {
        nixpkgs.config.allowUnfree = true;

        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "screenland";
          version = "0.1.0";
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
            outputHashes = {
              "iced_helper-0.1.0" = "sha256-Uxrv0YilON+pOEDaJBltjltU5cTmO2d3bdvIWRh/Zts=";
              "iced-0.14.0" = "sha256-hCK2QHrhGdwWaioa0hI4niHTad39g3mYsZe8ltcDXxY=";
              "iced_exdevtools-0.16.0" = "sha256-ITy16MwervR9vDig0kJZOtS6czXz2x2Xhx4YfsmhB40=";
            };
          };

          nativeBuildInputs = with pkgs; [ pkg-config makeWrapper ];

          inherit buildInputs;

          postInstall = ''
            wrapProgram $out/bin/screenland \
              --prefix LD_LIBRARY_PATH : ${pkgs.lib.makeLibraryPath buildInputs}
          '';
        };

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
