with (import <nixpkgs> {});
mkShell {
  buildInputs = [
    rustc
    cargo
    clippy
    rustfmt
    gdb
  ];
}
