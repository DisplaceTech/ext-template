#!/bin/sh
# shellcheck shell=sh
# (Not executable — sourced by bin/render and bin/sync.)
# Shared rendering helpers for bin/render and bin/sync. POSIX sh.
#
# Placeholders are UPPERCASE `{{LIKE_THIS}}` — deliberately disjoint from
# GitHub Actions' `${{ lowercase.expressions }}`, so workflow files render
# safely.

# Print the NAME value from a conf file (display only).
template_name() {
    sed -n 's/^NAME=//p' "$1" | head -1
}

# Build a sed script from KEY=VALUE conf lines. Values may contain spaces,
# `=`, and flag-like text; `\`, `|`, and `&` are escaped for sed.
_sed_script_from_conf() {
    conf=$1
    while IFS='=' read -r key value; do
        case "$key" in ''|\#*) continue ;; esac
        escaped=$(printf '%s' "$value" | sed -e 's/[\\|&]/\\&/g')
        printf 's|{{%s}}|%s|g\n' "$key" "$escaped"
    done < "$conf"
}

# Render every file under $1 into $2, substituting from conf $3.
# Fails loudly if any {{PLACEHOLDER}} survives — a missing conf key must
# never ship as literal mustache in a child repo.
render_tree() {
    src_root=$1
    dst_root=$2
    conf=$3

    script=$(mktemp)
    _sed_script_from_conf "$conf" > "$script"

    # find -print with newline separation is fine here: template file
    # paths are ours and never contain whitespace.
    find "$src_root" -type f | while IFS= read -r src; do
        rel=${src#"$src_root"/}
        dst=$dst_root/$rel
        mkdir -p "$(dirname "$dst")"
        sed -f "$script" "$src" > "$dst"
        if grep -nE '\{\{[A-Z_]+\}\}' "$dst" >&2; then
            echo "error: unrendered placeholder in $dst (missing key in $conf?)" >&2
            rm -f "$script"
            exit 1
        fi
        # Preserve executability (none of today's templates need it, but
        # a future templated script should survive the round trip). Plain
        # `[ -x ] && chmod` would leave the loop's exit status at 1 for
        # ordinary files and trip the `|| exit` below on the last file.
        if [ -x "$src" ]; then
            chmod +x "$dst"
        fi
    done || exit 1

    rm -f "$script"
}
