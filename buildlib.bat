cd crates/lib-seija
cargo build
echo "build success"
cd ../../target/debug/
mv liblib_seija.so ../../../../Scala/libseija-sn/liblib_seija.so
echo "move success"