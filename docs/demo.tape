Output demo.gif

Require echo
Require kanumi
Require fx
Require bat
Require jq

Set Shell "bash"
Set FontSize 16
Set Width 1200
Set Height 600

Sleep 1s

Type "kanumi" Sleep 500ms  Enter

Sleep 2s Ctrl+l Sleep 500ms

Type "kanumi ls" Sleep 500ms Enter

Sleep 2s Ctrl+l Sleep 500ms

Type "kanumi ls --json | fx" Sleep 500ms Enter

Sleep 1s Type "/tux" Sleep 500ms Enter Sleep 500ms

Sleep 2s Escape Sleep 500ms

Type "kanumi ls --tags linux" Sleep 500ms Enter

Sleep 3s Ctrl+l Sleep 500ms

Type "kanumi ls --scores favs=4.." Sleep 500ms Enter

Sleep 3s Ctrl+l Sleep 500ms

Type "kanumi cfg show --toml | bat --language toml" Sleep 500ms Enter

Sleep 2s Type G Sleep 2s Type q

Type "kanumi cfg show --json | jq --raw-output '.root_path'" Sleep 500ms Enter

Sleep 2s Ctrl+l Sleep 500ms

Sleep 1s
