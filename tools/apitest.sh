#!/bin/sh

api_url='http://127.0.0.1:8000'

# curl -vL \
#     -X POST \
#     "${api_url}/admin/migrate"

curl -vL \
    -X POST \
    -H 'Content-Type: application/json' \
    --data '{"name": "pinothink", "codename": "matsuba", "model": "Thinkpad T420"}' \
    "${api_url}/admin/device"

