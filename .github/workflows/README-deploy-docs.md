# Deploy Docs Workflow

Automatic deployment workflow for Starlight documentation to Vercel.

## Overview

The `deploy-docs.yml` workflow automatically builds and deploys the Starlight documentation site when changes are pushed to the main branch.

**Triggers:**
- Push to `main` branch with changes in `docs/starlight/**` directory
- Changes to `.github/workflows/deploy-docs.yml` itself
- Manual workflow dispatch via GitHub Actions UI

## Setup

### 1. Create Vercel Project

Set up a new Vercel project for the Starlight documentation:

```bash
cd docs/starlight
vercel
```

Note the following values from the Vercel project setup:
- **Project ID** — shown in Vercel dashboard under project settings
- **Org ID** — your Vercel organization ID

### 2. Generate Vercel Token

1. Go to https://vercel.com/account/tokens
2. Create a new token (scope: Full Account)
3. Copy the token value

### 3. Add GitHub Secrets

Add the following secrets to your GitHub repository (Settings → Secrets and variables → Actions):

| Secret | Value |
|--------|-------|
| `VERCEL_TOKEN` | Token from Vercel account |
| `VERCEL_ORG_ID` | Vercel organization ID |
| `VERCEL_PROJECT_ID` | Project ID from Vercel dashboard |

### 4. Configure Vercel Project (Optional)

To use preview deployments for pull requests, link the repository in Vercel:

1. Go to project settings in Vercel dashboard
2. Under "Git", connect the GitHub repository
3. Set production branch to `main`
4. Configure preview deployments for pull requests

## Workflow Steps

1. **Checkout** — Clones the repository code
2. **Setup Node.js** — Installs Node.js 20
3. **Install dependencies** — Runs `npm install` in `docs/starlight/`
4. **Build Starlight** — Runs `npm run build` to generate static files
5. **Deploy to Vercel** — Pushes the `dist/` directory to Vercel

## Manual Trigger

To manually trigger the deployment without pushing code:

1. Go to Actions → Deploy Docs
2. Click "Run workflow"
3. Select the branch and click the green button

## Troubleshooting

### Build Fails

Check the workflow logs in GitHub Actions:

1. Go to Actions → Deploy Docs
2. Click the failed workflow run
3. Expand the step that failed and review the error message

Common issues:
- Missing `npm run build` script in `package.json`
- Missing dependencies — check `package.json` and `package-lock.json`
- Node version incompatibility — ensure `package.json` targets Node 20+

### Deployment Fails

Verify Vercel secrets are set correctly:

```bash
# Verify secrets exist (from repo settings)
curl -H "Authorization: token $GITHUB_TOKEN" \
  https://api.github.com/repos/OWNER/REPO/actions/secrets
```

Common issues:
- Expired or invalid `VERCEL_TOKEN`
- Incorrect `VERCEL_PROJECT_ID` or `VERCEL_ORG_ID`
- Vercel project no longer exists

### Workflow Not Triggering

Check path filters — the workflow only runs when files in these paths change:
- `docs/starlight/**`
- `.github/workflows/deploy-docs.yml`

To test, you can use "Run workflow" under the workflow file.

## Monitoring

- **Vercel Dashboard** — https://vercel.com/dashboard
- **GitHub Actions** — Repository → Actions → Deploy Docs
- **Starlight Site** — Check deployment link in Vercel project

## Related Files

- Workflow: `.github/workflows/deploy-docs.yml`
- Starlight config: `docs/starlight/astro.config.mjs`
- Build script: `docs/starlight/package.json`
