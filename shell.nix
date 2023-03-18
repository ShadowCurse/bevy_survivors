{ pkgs ? import <nixpkgs> { } }:
let
  libs = with pkgs; [
    udev
    alsaLib
    wayland
    pkgconfig
    libxkbcommon
    vulkan-loader
  ];
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    mold
    clang
  ] ++ libs;
  shellHook = ''export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${with pkgs; lib.makeLibraryPath libs }"'';
}
