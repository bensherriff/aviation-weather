#!/bin/bash

# Usage: ./generate_cert.sh example.com
if [ "$#" -lt 1 ]; then
  echo "Usage: $0 <domain>"
  exit 1
fi

DOMAIN=$1
DAYS=365
SSL_DIR="./ssl"

# Create directory if it doesn't exist
mkdir -p "$SSL_DIR"

KEY_PATH="${SSL_DIR}/${DOMAIN}.key"
CRT_PATH="${SSL_DIR}/${DOMAIN}.crt"

echo "Generating self-signed certificate for ${DOMAIN}..."

openssl req -x509 -nodes -days ${DAYS} -newkey rsa:2048 \
  -keyout "${KEY_PATH}" \
  -out "${CRT_PATH}" \
  -subj "/CN=${DOMAIN}"

if [ $? -eq 0 ]; then
  echo "Successfully generated certificate:"
  echo "Private Key: ${KEY_PATH}"
  echo "Certificate: ${CRT_PATH}"
else
  echo "Certificate generation failed."
fi