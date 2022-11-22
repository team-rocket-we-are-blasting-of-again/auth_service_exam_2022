command -v openssl >/dev/null 2>&1 || { echo >&2 "openssl not installed.  Aborting."; exit 1; }

if [[ ! -d "./keys" ]]
then
    echo "keys folder not found, creating..."
    mkdir ./keys
fi

cd ./keys

openssl genrsa -out private.pem 2048
openssl rsa -in private.pem -outform PEM -pubout -out public.pem

echo "keys created"