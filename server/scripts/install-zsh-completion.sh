#!/bin/zsh

COMPLETION_SCRIPT_RELATIVE_PATH="scripts/.cargo_custom_completion.zsh"

echo "
# Custom completion for Track Server
chpwd_track_server() {
    if [[ \$PWD == *\"$(pwd)\"* ]]; then
        source $(pwd)/$COMPLETION_SCRIPT_RELATIVE_PATH
        functions[_cargo]=_cargo_custom
    else
        functions[_cargo]=\$functions[_cargo_original]
    fi
}

# Backup the original _cargo
if [ -z \$functions[_cargo_original] ]; then
    functions[_cargo_original]=\$functions[_cargo]
fi

# Add the custom chpwd function to the chpwd_functions array, which contains functions to be called on directory change
if (( \${+functions[chpwd]} )); then
    chpwd_functions+=(chpwd_track_server)
else
    function chpwd() {
        chpwd_track_server
    }
fi
" >~/.config_track_server
chmod +x ~/.config_track_server

echo "
# Custom completion for Track Server
source ~/.config_track_server
" >>~/.zshrc

echo "Custom Zsh completion for Track Server has been installed. Restart your terminal or run 'source ~/.zshrc'."
