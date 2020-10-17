with (import <nixpkgs> {});
mkShell {
  buildInputs = [
    cargo
    rustfmt
  ];
}
