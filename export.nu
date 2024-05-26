def export-modes [] {["release", "debug"]}

def targets [] {["windows", "linux"]}

def main [--mode (-m): string@export-modes, --target (-t): string@targets] {
    mut target = $target
    if $target == null {
        if (try {not not $env.OS} catch {false}) {
            $target = "windows"
        } else {
            $target = "linux"
        }
    }

    mut mode = $mode
    if $mode == null {
        $mode = "release"
    }

    cd rust

    let arg = if $mode == "release" {
        "--release"
    } else {
        ""
    }
    cargo build $arg

    cd ../godot

    let arg = match $target {
        "windows" => {
            "Windows Desktop"
        }
        "linux" => {
            "Linux/X11"
        }
    }


    mkdir $"export/($target)/($mode)"
        let exe = if $target == "windows" {
            "Chess.exe"
        } else {
            "Chess"
        }
    if (which godot | is-empty) {
        alias godot = godot4
        godot $"--export-($mode)" $arg $"export/($target)/($mode)/($exe)"
    }
    cd $"export/($target)/($mode)"
    let lib = if $target == "windows" {
        "chess.dll"
    } else {
        "libchess.so"
    }
    chmod +x $exe
    
    tar -a -c -f Chess.zip $exe Chess.pck $lib
}
