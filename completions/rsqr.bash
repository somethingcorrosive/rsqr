_rsqr() {
    local i cur prev opts cmd
    COMPREPLY=()
    if [[ "${BASH_VERSINFO[0]}" -ge 4 ]]; then
        cur="$2"
    else
        cur="${COMP_WORDS[COMP_CWORD]}"
    fi
    prev="$3"
    cmd=""
    opts=""

    for i in "${COMP_WORDS[@]:0:COMP_CWORD}"
    do
        case "${cmd},${i}" in
            ",$1")
                cmd="rsqr"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        rsqr)
            opts="-h -V --me --invert --no-quiet --charset --png --scale --help --version [TEXT]"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --charset)
                    COMPREPLY=($(compgen -W "block hash dot" -- "${cur}"))
                    return 0
                    ;;
                --png)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --scale)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
    esac
}

if [[ "${BASH_VERSINFO[0]}" -eq 4 && "${BASH_VERSINFO[1]}" -ge 4 || "${BASH_VERSINFO[0]}" -gt 4 ]]; then
    complete -F _rsqr -o nosort -o bashdefault -o default rsqr
else
    complete -F _rsqr -o bashdefault -o default rsqr
fi
