{
  description = "rust dev shell for sure";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
    }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
      };
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        packages = with pkgs; [
          (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)

          bacon

          cargo-msrv

          nixfmt
          nixd
        ];
        env = {
          RUST_BACKTRACE = "1";
          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        };

        shellHook = ''
          export PS1="\[\e[1;33m\][sure]\[\e[0m\] $PS1"

          alias msrv="cargo msrv find --linear -- cargo test"
          alias bench="cargo run --package sure-run-bench"

          mir() {
            local filter="''${1:?}"
            cargo clean -p sure --profile dev || return
            CARGO_INCREMENTAL=0 cargo rustc -p sure --lib -- \
              "-Zdump-mir=$filter" \
              -Zdump-mir-dir=target/mir
          }

          # e.g. mircd u8_add dumps both CODOMAIN and its codomain helper to target/mir.
          mircd() {
            mir "''${1:?}::CODOMAIN|''${1:?}::codomain"
          }
        '';
      };
    };
}
