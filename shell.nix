with import <nixpkgs> {};
stdenv.mkDerivation {
  name = "vwb-env";
  buildInputs = [ gcc wasm-pack yarn ];
}