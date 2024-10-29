#!/bin/bash
SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)

# This script converts the protocol buffer files to python
OUT_DIR="$SCRIPT_DIR/src/aristech_stt_client/proto"

# Directory containing the proto files
PROTO_DIR="$SCRIPT_DIR/../protos"

rm -rf ${OUT_DIR}
mkdir -p ${OUT_DIR}

# Generate python from proto files using grpcio-tools
python -m grpc_tools.protoc \
  -I ${PROTO_DIR} \
  --proto_path=${PROTO_DIR} \
  --python_out=${OUT_DIR} \
  --grpc_python_out=${OUT_DIR} \
  --plugin=protoc-gen-mypy=$(which protoc-gen-mypy) \
  --mypy_out=${OUT_DIR} \
  ${PROTO_DIR}/stt_service.proto

# Replace the import path in the generated files
sed -i '' 's/import stt_service_pb2 as stt__service__pb2/from . import stt_service_pb2 as stt__service__pb2/g' ${OUT_DIR}/stt_service_pb2_grpc.py

# Copy the __init__.py file to the generated directory
touch ${OUT_DIR}/__init__.py
