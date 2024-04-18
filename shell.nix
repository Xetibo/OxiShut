{ pkgs ? import <nixpkgs> {} }:

with pkgs;
mkShell {
  nativeBuildInputs = [
    pkg-config
  ];

  buildInputs = [
    dbus
    gtk4
    gtk4-layer-shell
  ];

}
