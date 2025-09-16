{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs =
    {
      nixpkgs,
      flake-utils,
      fenix,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ fenix.overlays.default ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            (fenix.packages.${system}.complete.withComponents [
              "cargo"
              "clippy"
              "rustc"
              "rustfmt"
            ])
            rust-analyzer
            # rust-analyzer-nightly # If you prefer a nightly version
            nil # Nix Language Server for Nix files
            nixfmt-rfc-style # Nix code formatter
            taplo # TOML formatter (for Cargo.toml)
            # Add any other development tools here
            # For example: git, editorconfig-checker
          ];

          # Optional: Set environment variables for the dev shell
          # shellHook = ''
          #   export RUST_LOG=debug
          # '';
        };
      }
    );
}
