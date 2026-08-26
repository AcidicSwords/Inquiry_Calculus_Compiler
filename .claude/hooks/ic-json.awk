# ic-json.awk -- depth-aware extractor for one JSON string value.
#
# Usage:  awk -v want=tool_input.command -f ic-json.awk < hook-input.json
#
# Prints the decoded string value of the first path match, or nothing.
# A real scanner is used rather than a regex so that a *value* containing
# `"file_path":` (e.g. the content of a Write) can never be mistaken for
# structure.  \uXXXX collapses to `?`; the guard only matches ASCII.

{ buf = buf $0 "\n" }

function pathof(d,   k, p) {
    p = ""
    for (k = 1; k <= d; k++) {
        if (key[k] == "") continue
        p = (p == "" ? key[k] : p "." key[k])
    }
    return p
}

END {
    n = length(buf)
    depth = 0
    i = 1
    while (i <= n) {
        c = substr(buf, i, 1)

        if (c == "\"") {
            s = ""
            i++
            while (i <= n) {
                ch = substr(buf, i, 1)
                if (ch == "\\") {
                    nx = substr(buf, i + 1, 1)
                    if (nx == "n") s = s "\n"
                    else if (nx == "t") s = s "\t"
                    else if (nx == "r") s = s "\r"
                    else if (nx == "b") s = s "\b"
                    else if (nx == "f") s = s "\f"
                    else if (nx == "u") { s = s "?"; i += 4 }
                    else if (nx == "\\" || nx == "\"" || nx == "/") s = s nx
                    # An invalid escape keeps its backslash, so a Windows path
                    # survives whether or not it arrived properly doubled.
                    else s = s "\\" nx
                    i += 2
                    continue
                }
                if (ch == "\"") { i++; break }
                s = s ch
                i++
            }
            j = i
            while (j <= n && substr(buf, j, 1) ~ /[ \t\n\r]/) j++
            if (substr(buf, j, 1) == ":") {
                key[depth] = s
                i = j + 1
            } else if (pathof(depth) == want) {
                printf "%s", s
                exit
            }
            continue
        }

        if (c == "{" || c == "[") { depth++; i++; continue }
        if (c == "}" || c == "]") { delete key[depth]; depth--; i++; continue }

        # Hook payloads use booleans for fields such as stop_hook_active.
        # Return JSON literals only when the scanner is positioned at the
        # value of the exact requested path; text inside strings is handled
        # by the string branch above and cannot be mistaken for structure.
        if (pathof(depth) == want) {
            rest = substr(buf, i)
            if (rest ~ /^true([^A-Za-z0-9_]|$)/) { printf "true"; exit }
            if (rest ~ /^false([^A-Za-z0-9_]|$)/) { printf "false"; exit }
            if (rest ~ /^null([^A-Za-z0-9_]|$)/) { printf "null"; exit }
            if (match(rest, /^-?[0-9]+([.][0-9]+)?([eE][+-]?[0-9]+)?/)) {
                printf "%s", substr(rest, 1, RLENGTH)
                exit
            }
        }
        i++
    }
}
