{
  description = "Rust dev flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils = {
      url = "github:numtide/flake-utils";
    };
  };

  outputs =
    { nixpkgs, ... }@inputs:
    inputs.flake-utils.lib.eachDefaultSystem (system: 
    let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ (import inputs.rust-overlay) ];
      };
    in
    {
      devShells.default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          (rust-bin.stable.latest.default.override {
            extensions = ["rust-src"];
          })
          rust-analyzer
          clippy
          sccache
          bacon
          lldb
          pkg-config
          fontconfig
        ];

        shellHooks = ''
          export RUSTC_WRAPPER="$(which sccache)"
          echo "You were warped to Rust shell"
        '';
      };
    });
}
