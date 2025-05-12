{
    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
        flake-utils.url = "github:numtide/flake-utils";

        rust-overlay = {
            url = "github:oxalica/rust-overlay";
            inputs.nixpkgs.follows = "nixpkgs";
        };
    };

    nixConfig = {
        extra-substituters = [
            "https://cache.nixos.org"
            "https://nix-community.cachix.org"
            "https://an-anime-team.cachix.org"
        ];

        extra-trusted-public-keys = [
            "cache.nixos.org-1:6NCHdD59X431o0gWypbMrAURkbJ16ZPMQFGspcDShjY="
            "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
            "an-anime-team.cachix.org-1:nr9QXfYG5tDXIImqxjSXd1b6ymLfGCvviuV8xRPIKPM="
        ];
    };

    outputs = { self, nixpkgs, flake-utils, rust-overlay }:
        flake-utils.lib.eachDefaultSystem (system:
                let
                    pkgs = import nixpkgs {
                        inherit system;

                        overlays = [ rust-overlay.overlays.default ];
                    };

                    config = pkgs.lib.importTOML ./Cargo.toml;

                in {
                    packages.default = pkgs.rustPlatform.buildRustPackage {
                        pname = config.package.name;
                        version = config.package.version;

                        src = ./.;
                        cargoLock.lockFile = ./Cargo.lock;

                        doCheck = false;

                        meta = with pkgs.lib; {
                            description = config.package.description;
                            homepage = config.package.homepage;
                            license = licenses.gpl3Only;

                            maintainers = [
                                {
                                    name = "Nikita Podvirnyi";
                                    email = "krypt0nn@vk.com";
                                    matrix = "@krypt0nn:mozilla.org";
                                    github = "krypt0nn";
                                    githubId = 29639507;
                                }
                            ];
                        };

                        nativeBuildInputs = with pkgs; [
                            rust-bin.stable.latest.minimal
                            gcc
                        ];
                    };

                    devShells.default = pkgs.mkShell {
                        nativeBuildInputs = with pkgs; [
                            (rust-bin.stable.latest.default.override {
                                extensions = [ "rust-src" ];
                            })

                            gcc
                            cmake
                            pkg-config

                            git
                            unzip
                            p7zip
                            libwebp
                        ];

                        buildInputs = with pkgs; [
                            gtk4
                            glib
                            gdk-pixbuf
                            gobject-introspection
                            libadwaita
                            openssl
                            protobuf
                        ];
                    };
                });
}
