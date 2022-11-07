with import <nixpkgs> {};
stdenv.mkDerivation {
  name = "vwb-env";
  buildInputs = [ gcc pkg-config openssl wasm-pack yarn ];
}