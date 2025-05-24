#! /bin/sh

if [ -z "$INJECTION_ENV_PREFIX" ]; then
    echo "INJECTION_ENV_PREFIX is not set. Exiting."
    exit 1
fi

for i in $(env | grep "^$INJECTION_ENV_PREFIX"); do
    key=$(echo "$i" | cut -d '=' -f 1)
    value=$(echo "$i" | cut -d '=' -f 2-)

    echo "$key=$value"

    find "/usr/src/app/server/chunks/" -type f -exec sed -i 's|'"${key}"'|'"${value}"'|g' {} \;
    find "/usr/src/app/server/_app/immutable/assets/" -type f -exec sed -i 's|'"${key}"'|'"${value}"'|g' {} \;
    find "/usr/src/app/client/_app/immutable/chunks/" -type f -exec sed -i 's|'"${key}"'|'"${value}"'|g' {} \;
    find "/usr/src/app/client/_app/immutable/assets/" -type f -exec sed -i 's|'"${key}"'|'"${value}"'|g' {} \;
done

exec bun run ./index.js