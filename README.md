# kjhjason.com

This is the source code for my personal website, [kjhjason.com](https://kjhjason.com).

Note: The website design is inspired by [leerob/leerob.io](https://github.com/leerob/leerob.io).

## Tech Stack

### Backend

- [Actix Web](https://github.com/actix/actix-web) (Rust)
- [MongoDB Atlas](https://www.mongodb.com/)

### Frontend

- [Askama](https://github.com/djc/askama) Templates
  - Pre-compiled templates for server-side rendering.
- [Tailwind CSS](https://tailwindcss.com/)
  - [DaisyUI](https://daisyui.com/)
- [htmx](https://github.com/bigskysoftware/htmx)
- [pdf.js](https://github.com/mozilla/pdf.js)
  - Mainly for previewing my resume

### Deployment

- [Docker](https://www.docker.com/)
- [Cloudflare](https://www.cloudflare.com/)
  - [R2](https://www.cloudflare.com/developer-platform/r2/) for storing blog files like images
  - Reverse Proxy for bot protection and caching
- [Fly.io](https://fly.io/)

### Security

- Automated attacks are mitigated using Cloudflare's [turnstile](https://www.cloudflare.com/products/turnstile/) CAPTCHA solution.
- Passwords are hashed using [Argon2](https://github.com/RustCrypto/password-hashes/tree/master/argon2).
- TOTP secrets for 2FA are encrypted using [XChaCha20-Poly1305](https://github.com/RustCrypto/AEADs/tree/master/chacha20poly1305) before being stored in the database.
- Nonces are generated with `rand::thread_rng()` that is cryptographically secure and are usually 32 bytes long.
- Implemented various middleware for enhanced security to adhere to [OWASP Top 10](https://owasp.org/www-project-top-ten/) guidelines:
  - Content Security Policy.
  - Cross-Site Request Forgery.
    - Uses HMAC-SHA1 for CSRF tokens via [hmac-serialiser](https://github.com/KJHJason/hmac-serialiser/tree/master/rust).
  - HTTP Strict Transport Security.
  - Authentication using HMAC-SHA512 via [hmac-serialiser](https://github.com/KJHJason/hmac-serialiser/tree/master/rust) for a shorter but secure tokens instead of using JSON Web Tokens (JWT).
    - [hmac-serialiser](https://github.com/KJHJason/hmac-serialiser/tree/master/rust) uses implementations by [RustCrypto](https://github.com/RustCrypto).
