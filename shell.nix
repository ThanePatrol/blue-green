let
  nixpkgs = import <nixpkgs> { };
in
  with nixpkgs;
  stdenv.mkDerivation {
    allowUnsupportedSystem = true;
    name = "moz_overlay_shell";
    buildInputs = [
      wasm-bindgen-cli
      simple-http-server
      openssl
      pkg-config
      libsodium
      zlib
      binutils
      lld_9
      qemu

    ];

  shellHook = ''
  '';
  }

