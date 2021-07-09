#!/usr/bin/bash

export UNAGI_PORTAL_PASSWORD="$(
    echo 'U2FsdGVkX1+oVx4ISl/n2AAvRfJvfqTVxoLyMzUlYb8=' |
        openssl enc -d -pbkdf2 -des -base64 -k "${UNAGI_PASSWORD}"
)"
export UNAGI_API_KEY="$(
    echo 'U2FsdGVkX1+037TuczmfVSUAAxXlD0OqeAa50aSl/tF0NfgumZnv4g89A8ggJHL9cATCnlUsLyI=' |
        openssl enc -d -pbkdf2 -des -base64 -k "${UNAGI_PASSWORD}"
)"

export SQL_ADDRESS=34.146.137.27
export SQL_USER=root
export SQL_DATABASE=database
export SQL_PASSWORD="${UNAGI_PASSWORD}"

exec /usr/local/bin/server --logtostderr "$@"
