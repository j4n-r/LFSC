{
  description = "lfsc";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.systems.url = "github:nix-systems/default";
  inputs.flake-utils = {
    url = "github:numtide/flake-utils";
    inputs.systems.follows = "systems";
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      self,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            cargo
            rustfmt
            clippy
            rustc
            sqlx-cli

            openssl
            pkg-config

            sqlite

            websocat
            jq

            pkgs.python312
            pkgs.python312Packages.websockets
          ];
          shellHook = ''
            mkdir -p instance
            if [ ! -f instance/dev.sqlite3 ]; then
               echo
               echo "Initializing SQLite database at instance/dev.sqlite3"  
               sqlite3 instance/dev.sqlite3 < instance/init.sql
               echo
            fi
          '';
          env = {
            DATABASE_URL = "sqlite:../sc-admin/instance/db.sqlite3"; # relative path from sc-core/
            PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
          };
        };
        apps.cargo-build = {
          type = "app";
          program =
            (pkgs.writeShellScript "" ''
              cd git rev-parse --show-toplevel
              cd sc-core
              cargo build --release
              mkdir -p ../sc-admin/ws-server
              cp ./target/release/lfsc ../sc-admin/ws-server/lfsc-${system}
            '').outPath;
        };
      }
    );
}
