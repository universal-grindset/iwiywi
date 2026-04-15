# Hosting iwiywi on your own VPS

`iwiywi serve` boots an HTTP server that renders the same pulse the TUI does,
so any browser on your phone or laptop can open it. One binary, no extra
services required — you just need somewhere to run the process and (ideally)
a reverse proxy for TLS.

## Option A — Docker

```sh
# On your VPS, clone or copy the repo, then:
docker build -t iwiywi .
docker volume create iwiywi-data

# Seed config + secrets into the volume (one-time).
docker run --rm -it -v iwiywi-data:/data alpine sh
  # inside the container:
  mkdir -p /data/.iwiywi
  cat > /data/.iwiywi/config.toml <<'TOML'
  [ai]
  model = "anthropic/claude-haiku-4-5"
  gateway_url = "https://ai-gateway.vercel.sh/v1"
  TOML
  cat > /data/.iwiywi/.env <<'ENV'
  VERCEL_AI_GATEWAY_TOKEN=sk-...
  ENV
  exit

# Run the server.
docker run -d --name iwiywi \
  --restart unless-stopped \
  -v iwiywi-data:/data \
  -p 8080:8080 \
  iwiywi

# Schedule the 6am fetch — run a sibling one-shot on cron:
#   0 6 * * * docker exec iwiywi iwiywi fetch
```

## Option B — systemd on the host

```sh
# Install the binary (or copy it over from a build host).
cargo install --git https://github.com/universal-grindset/iwiywi --locked
sudo cp ~/.cargo/bin/iwiywi /usr/local/bin/iwiywi

# Put config + secrets under the service user's home.
sudo useradd --system --home /var/lib/iwiywi --create-home iwiywi
sudo -u iwiywi mkdir -p /var/lib/iwiywi/.iwiywi
sudo -u iwiywi tee /var/lib/iwiywi/.iwiywi/config.toml <<'TOML'
[ai]
model = "anthropic/claude-haiku-4-5"
gateway_url = "https://ai-gateway.vercel.sh/v1"
TOML
sudo -u iwiywi tee /var/lib/iwiywi/.iwiywi/.env <<'ENV'
VERCEL_AI_GATEWAY_TOKEN=sk-...
ENV
sudo chmod 600 /var/lib/iwiywi/.iwiywi/.env

# Install the units.
sudo cp deploy/iwiywi.service /etc/systemd/system/
sudo cp deploy/iwiywi-fetch.service /etc/systemd/system/
sudo cp deploy/iwiywi-fetch.timer /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable --now iwiywi.service iwiywi-fetch.timer
```

## Putting it on the public internet

`iwiywi serve` speaks plain HTTP and has no auth of its own. Pick one:

- **Caddy** (easiest, auto-HTTPS):
  ```
  pulse.example.com {
    reverse_proxy localhost:8080
    basicauth {
      you JDJhJDEwJEVCN...   # output of `caddy hash-password`
    }
  }
  ```
- **nginx + certbot** with `auth_basic` and `proxy_pass http://127.0.0.1:8080`.
- **Tailscale / WireGuard**: bind to `127.0.0.1:8080` and reach it over the
  tunnel from any device — no public exposure at all.
- **Cloudflare Tunnel**: zero-config ingress with Access in front for auth.

Whatever you use, keep AI credentials and classified readings behind *some*
gate — the server itself is intentionally unauthenticated so the host has
exactly one job.

## Env vars the web server honors

Same as the TUI — set them in `~/.iwiywi/.env` or the systemd unit:

| Var | What it does |
|---|---|
| `IWIYWI_PULSE_SECS` | initial auto-advance interval (0 = manual) |
| `IWIYWI_SOBER_SINCE` | `YYYY-MM-DD`, exposed as sobriety days in `/api/items` |
| `VERCEL_AI_GATEWAY_TOKEN` / `AZURE_OPENAI_API_KEY` | used by `iwiywi fetch` |

`IWIYWI_THEME`, `IWIYWI_PALETTE`, `IWIYWI_PATTERN` are terminal-only; the
web page follows your OS light/dark preference.
