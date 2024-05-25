def main [] {
    cd rust
    cargo build --release
    cd ../godot
    mkdir export
    godot --export-release "Windows Desktop" export/Chess.exe
    cd export
    tar -a -c -f Chess.zip Chess.exe chess.dll Chess.pck
}
