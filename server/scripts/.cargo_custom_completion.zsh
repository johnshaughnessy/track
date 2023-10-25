# .cargo_custom_completion.zsh
_cargo_custom() {
    local -a _custom_cargo_commands
    _custom_cargo_commands=(
        'gen-db:Generate database'
    )
    _describe 'command' _custom_cargo_commands
}
