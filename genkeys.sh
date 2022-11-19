if [[ ! -d "./keys" ]]
then
    echo "keys folder not found, creating..."
    mkdir ./keys
fi

cd ./keys

openssl genrsa -out private.pem 2048
openssl rsa -in private.pem -outform PEM -pubout -out public.pem

echo "keys created"