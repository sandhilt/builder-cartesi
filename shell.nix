{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    libclang
 ];

  shellHook = ''
    echo "Welcome"
  '';
}
