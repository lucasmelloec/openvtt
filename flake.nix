{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
        rustToolchain = (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml).override {
          targets = [ "wasm32-unknown-unknown" ];
        };
      in {
        formatter = pkgs.nixfmt-classic;
        devShells.default = pkgs.mkShell rec {
          packages = with pkgs; [
            rustToolchain
            rust-analyzer-unwrapped
            pkg-config
            clang
            lld
            linuxPackages_latest.perf
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
            # Others
            just
            systemfd
          ];

          env = {
            # Required by rust-analyzer
            RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
            LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath packages;
          };
          shellHook = ''
          export PATH="$PATH:$HOME/.cargo/bin"
          '';
        };
      });
}
