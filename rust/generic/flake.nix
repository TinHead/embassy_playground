{
  description = "A devShell example";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            python39
            openssl
            pkgconfig
            exa
            fd
            gdb
            rust-analyzer
            lldb
            (rust-bin.stable.latest.default.override {
              extensions = [ "rust-src" ];
              targets = [ "thumbv6m-none-eabi" ]; #"arm-unknown-linux-gnueabihf" ];
            })
          ];

          shellHook = ''
            alias ls=exa
            alias find=fd
          '';
        };
      }
    );
}
