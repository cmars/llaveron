{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {
  nativeBuildInputs = with pkgs.buildPackages; [
    rustup
    nettle
    pkg-config
    llvmPackages_latest.llvm
    llvmPackages_latest.clang
    llvmPackages_latest.libclang
  ];

  shellHook = ''
    export LIBCLANG_PATH=$(nix eval --raw nixpkgs#llvmPackages_latest.libclang.lib)/lib
  '';
}

