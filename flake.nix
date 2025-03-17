{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      fenix,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        target = "wasm32-unknown-unknown";
        toolchain =
          with fenix.packages.${system};
          combine [
            (complete.withComponents [
              "cargo"
              "clippy"
              "rustc"
              "rustfmt"
              "rust-analyzer"
            ])
            targets.${target}.latest.rust-std
          ];
      in
      {
        formatter = pkgs.nixfmt-rfc-style;
        devShells.default = pkgs.mkShell rec {
          packages = with pkgs; [
            toolchain
            pkg-config
            clang
            lld
            linuxPackages_latest.perf
          ];

          buildInputs = with pkgs; [
            # Needed by Bevy
            udev
            alsa-lib
            vulkan-loader
            # Bevy x11 feature
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
            # Bevy wayland feature
            libxkbcommon
            wayland
          ];

          env = {
            RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";
            LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
          };

          shellHook = ''
            export PATH="$PATH:$HOME/.cargo/bin"
          '';
        };
      }
    );
}
