FROM nginx:stable

RUN apt-get update \
    && apt-get install -y cron certbot \
    && apt-get update -y \
    && mkdir -p /scripts \
    && rm -f /etc/nginx/conf.d/default.conf

# Install a Crontab entry which:
# - Sleep for a random interval between 0 and 60 minutes.
# - Runs "certbot renew" at 12:00pm UTC every day.
RUN echo "0 12 * * * sleep $(( $(od -N1 -tuC -An /dev/urandom) % 60 ))m; root /scripts/renew-certs.sh" >/etc/cron.d/certbot-renew

COPY ./scripts /scripts/
RUN chmod +x /scripts/*.sh
WORKDIR /scripts

# Run both nginx and cron together
CMD [ "sh", "-c", "cron && nginx -g 'daemon off;'" ]