{
    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs/nixos-24.05";
        nixpkgs-unstable.url = "github:nixos/nixpkgs/nixos-unstable";
    };

    outputs = { self, nixpkgs, nixpkgs-unstable }:
        let
            pkgs = nixpkgs.legacyPackages.x86_64-linux;
            pkgs-unstable = nixpkgs-unstable.legacyPackages.x86_64-linux;

        in {
            devShells.x86_64-linux.default = pkgs.mkShell {
                nativeBuildInputs = with pkgs; [
                    pkgs-unstable.rustup
                    pkgs-unstable.rustfmt
                    pkgs-unstable.clippy

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
                ];
            };
        };
}
