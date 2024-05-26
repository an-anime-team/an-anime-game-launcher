let
    nixpkgs = builtins.fetchGit {
        name = "nixos-23.11";
        url = "https://github.com/nixos/nixpkgs";
        ref = "refs/heads/nixos-23.11";
    };

    nixpkgs-unstable = builtins.fetchGit {
        name = "nixos-unstable";
        url = "https://github.com/nixos/nixpkgs";
        ref = "refs/heads/nixos-unstable";
    };

    pkgs = import nixpkgs {};
    pkgs-unstable = import nixpkgs-unstable {};

in pkgs.mkShell {
    nativeBuildInputs = with pkgs; [
        pkgs-unstable.rustup
        pkgs-unstable.rustfmt
        pkgs-unstable.clippy

        gcc
        cmake
        pkg-config
    ];

    buildInputs = with pkgs; [
        gtk4
        glib
        gdk-pixbuf
        gobject-introspection

        libadwaita
    ];
}
