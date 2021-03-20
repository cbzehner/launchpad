# Accounts, Software & Tools

_Note: This guide choses to avoid solutions from Microsoft & Google for most services. These companies offer an integrated solution to many of the services selected here, but have a high degree of vendor lock-in and the quality of support varies significantly for each service._

_If preferring Microsoft or Google makes sense for your business please replace the recommended service with their equivalent from those providers._

## Account setup

1. Select a domain name and register your domain with Namecheap using your personal email. We'll change this at the last step.
1. Register for a Cloudflare account (again with your personal email) and add your new domain to your account.
1. Create an email account with Fastmail using `<yourname>@<yourdomain>` and follow the instructions to set up your custom domain.
1. Set up additional email aliases; `hello@`, `security@`, and `admin@` are good places to start.
1. Sign up for 1Password and save your three existing accounts; domain registrar (Namecheap), dns provider (Cloudflare), and email service (Fastmail).
1. Now go update your domain registrar (Namecheap) and dns provider (Cloudflare) to use your business emails.

This setup provides you a solid, self-contained and secure basis for moving forward with building your company. Anyone needing a company email can be added to your email service and you can select what passwords to share with them via the password manager.

## Software services

### Email

Setup an email account. This is prerequisite to managing most software.

I chose to use Fastmail for this purpose and set up my account using a custom domain at `launchpad.rs`.

### Password Manager

In order to secure your accounts and keep records it is _highly recommended_ to use a password manager for storing the credentials to the various services needed to run your business.

### Domain

A domain is required in order to have a presence on the internet.

Namecheap is a reputable domain provider. For https://launchpad.rs, Ninet was chosen due to it's ability to provide `.rs` domains.

### Domain Name System (DNS)

The [domain name system](https://www.cloudflare.com/learning/dns/what-is-dns/) is the heart of the internet. Throughout this guide Cloudflare will be used to manage the DNS records for our domain.

### Marketing website

### Mail provider

Postmark

### Hosting

Hertzner dedicated machine


