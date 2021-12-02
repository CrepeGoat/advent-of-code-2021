{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    
    # nativeBuildInputs is usually what you want -- tools you need to run
    nativeBuildInputs = [
        pkgs.python39
        pkgs.cargo
        pkgs.clippy
        pkgs.rustfmt
    ];
    shellHook =
      ''
        . venv/bin/activate
      '';
}
