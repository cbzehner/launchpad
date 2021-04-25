#!/bin/sh
set -Eefuo pipefail

# Run certbot renew, which scans data in the /etc/letsencrypt directory
# and from that determines which certificates need renewal. If any need
# renewal, it will initiate the process and write new certificates.
/usr/bin/certbot renew

# The script ends by sending a SIGHUP signal to the Nginx process.
# This will tell Nginx to reread its configuration files, and therefore
# start using them and any newly downloaded certificates.
kill -HUP `cat /var/run/nginx.pid`