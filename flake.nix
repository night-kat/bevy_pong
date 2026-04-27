{
  description = "bevy flake with sccache";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    bevy_cli.url = "github:TheBevyFlock/bevy_cli";
    sccache = {
      url = "github:mozilla/sccache";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    bevy_cli,
    sccache,
  }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
      overlays = [sccache.overlays.default];
    };
  in {
    devShells."x86_64-linux".default = pkgs.mkShell {
      packages = with pkgs; [
        cargo
        rustc
        rustfmt
        clippy
        rust-analyzer
        pkg-config
        bacon
        pkgs.sccache

        wayland
        # for Linux
        # Audio (Linux only)
        alsa-lib
        # Cross Platform 3D Graphics API
        vulkan-loader
        # For debugging around vulkan
        pkgs.vulkan-tools
        # Other dependencies
        libudev-zero
        libx11
        libxcursor
        libxi
        libxrandr
        libxkbcommon
      ];

      nativeBuildInputs = [pkgs.pkg-config];

      env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
        pkgs.vulkan-loader
        pkgs.libx11
        pkgs.libxi
        pkgs.libxcursor
        pkgs.libxkbcommon
      ];
      shellHook = ''
        export CARGO_CLIPPY_FLAGS='-- -W clippy::pedantic -W clippy::nursery -W clippy::unwrap_used'
      '';
    };
  };
}
