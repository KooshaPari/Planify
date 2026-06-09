#!/usr/bin/env bash
set -euo pipefail

python manage.py wait_for_db
# Wait for migrations
python manage.py wait_for_migrations
# Run the processes
celery -A plane beat -l info