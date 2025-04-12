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

# Define CA file names
CA_KEY="${SSL_DIR}/${SSL_CA_NAME}.key"
CA_CERT="${SSL_DIR}/${SSL_CA_NAME}.pem"

# Check if CA files exist; if not, generate them.
if [ ! -f "$CA_KEY" ] || [ ! -f "$CA_CERT" ]; then
  echo "Generating CA key and self-signed CA certificate..."
  openssl genrsa -out "$CA_KEY" 4096
  if [ $? -ne 0 ]; then
    echo "Failed to generate CA key"
    exit 1
  fi
  openssl req -x509 -new -nodes -key "$CA_KEY" -sha256 -days 1024 -out "$CA_CERT" -subj "/CN=My Custom CA"
  if [ $? -ne 0 ]; then
    echo "Failed to generate CA certificate"
    exit 1
  fi
  echo "CA generated successfully:"
else
  echo "Existing CA:"
fi
echo "  CA Private Key: $CA_KEY"
echo "  CA Certificate: $CA_CERT"

# Define domain file names
DOMAIN_KEY="${SSL_DIR}/${DOMAIN}.key"
DOMAIN_CSR="${SSL_DIR}/${DOMAIN}.csr"
DOMAIN_CERT="${SSL_DIR}/${DOMAIN}.crt"

echo "Generating private key for domain ${DOMAIN}..."
openssl genrsa -out "$DOMAIN_KEY" 2048
if [ $? -ne 0 ]; then
  echo "Failed to generate domain key"
  exit 1
fi

echo "Generating CSR for domain ${DOMAIN}..."
openssl req -new -key "$DOMAIN_KEY" -out "$DOMAIN_CSR" -subj "/CN=${DOMAIN}"
if [ $? -ne 0 ]; then
  echo "Failed to generate CSR for ${DOMAIN}"
  exit 1
fi

echo "Signing certificate for ${DOMAIN} using our CA..."
openssl x509 -req -in "$DOMAIN_CSR" -CA "$CA_CERT" -CAkey "$CA_KEY" -CAcreateserial -out "$DOMAIN_CERT" -days $DAYS -sha256
if [ $? -ne 0 ]; then
  echo "Failed to sign certificate for ${DOMAIN}"
  exit 1
fi

echo "Successfully generated the following files:"
echo "  CA Private Key: $CA_KEY"
echo "  CA Certificate: $CA_CERT"
echo "  Domain Private Key: $DOMAIN_KEY"
echo "  Domain Certificate: $DOMAIN_CERT"
echo "  Domain Certificate Signing Request: $DOMAIN_CSR"
