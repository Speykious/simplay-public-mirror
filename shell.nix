{ pkgs ? import <nixpkgs> { } }:
with pkgs;
mkShell {
  buildInputs = [
    cargo
    pkgconfig
    udev
    alsaLib
    lutris
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
    vulkan-tools
    vulkan-headers
    vulkan-loader
    vulkan-validation-layers
    clang
    lld
  ];
  shellHook = ''export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath [
    pkgs.alsaLib
    pkgs.udev
    pkgs.vulkan-loader
  ]}"'';
}
