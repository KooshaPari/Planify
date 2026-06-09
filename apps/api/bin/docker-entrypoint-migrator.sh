#!/usr/bin/env bash
set -euo pipefail

python manage.py wait_for_db $1

python manage.py migrate $1