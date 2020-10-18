with (import <nixpkgs> {});
mkShell {
  buildInputs = [
    cargo
    clippy
    rustfmt
  ];
}
