{
  description = "Screenland is a program for creating and editing screenshots";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
      crane,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          config.allowUnfree = true;
          overlays = [ (import rust-overlay) ];
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default;
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;
        src = ./.;

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
        commonArgs = {
          inherit src;

          nativeBuildInputs = with pkgs; [
            pkg-config
            makeWrapper
          ];

          inherit buildInputs;
        };
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
      in
      {
        packages.default = craneLib.buildPackage (
          commonArgs
          // {
            inherit cargoArtifacts;

            postInstall = ''
              wrapProgram $out/bin/screenland \
                --prefix LD_LIBRARY_PATH : ${pkgs.lib.makeLibraryPath buildInputs}
            '';
          }
        );

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
