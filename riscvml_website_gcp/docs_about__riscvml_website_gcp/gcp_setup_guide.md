# GCP Cloud Run Setup — riscvml.org

Step-by-step guide to deploy the Rust SPA webserver on Google Cloud Platform.

## Prerequisites

- Google account
- Credit card (for GCP billing — Cloud Run has a generous free tier)
- `gcloud` CLI installed locally
- Docker installed locally (optional, for local testing)

---

## Step 1: Create a GCP Project

1. Go to https://console.cloud.google.com
2. Click the project dropdown (top-left) → **New Project**
3. **Project name:** `riscvml-website`
4. **Organization:** leave as-is (or select yours)
5. Click **Create**
6. Note the **Project ID** (e.g., `riscvml-website` or `riscvml-website-123456`)

## Step 2: Enable Billing

1. Go to **Billing** in the left sidebar
2. Link a billing account (or create one with your credit card)
3. Cloud Run free tier: **2 million requests/month** + **360,000 GB-seconds** free

## Step 3: Install & Configure gcloud CLI

```bash
# Install gcloud (if not already installed)
# Ubuntu/Debian:
sudo apt-get install google-cloud-cli

# Or download from: https://cloud.google.com/sdk/docs/install

# Login
gcloud auth login

# Set the project
gcloud config set project riscvml-website

# Set default region (Belgium — close to Germany)
gcloud config set run/region europe-west1
```

## Step 4: Enable Required APIs

```bash
gcloud services enable \
  run.googleapis.com \
  cloudbuild.googleapis.com \
  artifactregistry.googleapis.com
```

## Step 5: Deploy to Cloud Run (from source)

```bash
cd /home/maxx/Dropbox/scottsoft_sdn/src/riscvml/riscvml_website_gcp

# Deploy directly from source — GCP builds the Docker image for you
gcloud run deploy riscvml-web \
  --source . \
  --region europe-west1 \
  --allow-unauthenticated \
  --port 8080 \
  --memory 256Mi \
  --cpu 1 \
  --min-instances 0 \
  --max-instances 10 \
  --concurrency 80
```

GCP will:
1. Upload your source code
2. Build the Docker image using your `Dockerfile`
3. Push the image to Artifact Registry
4. Deploy to Cloud Run

You'll get a URL like: `https://riscvml-web-xxxxx-ew.a.run.app`

## Step 6: Test the Deployment

```bash
# Health check
curl https://riscvml-web-xxxxx-ew.a.run.app/api/health
# → {"status":"ok","version":"0.1.0"}

# Modules API
curl https://riscvml-web-xxxxx-ew.a.run.app/api/modules
# → JSON array of modules

# Open in browser
xdg-open https://riscvml-web-xxxxx-ew.a.run.app
```

## Step 7: Map Custom Domain (riscvml.org)

### Option A: Cloud Run Domain Mapping (simplest)

```bash
# Map riscvml.org to your Cloud Run service
gcloud run domain-mappings create \
  --service riscvml-web \
  --domain riscvml.org \
  --region europe-west1
```

GCP will give you DNS records to add:

1. Go to your domain registrar (wherever you bought riscvml.org)
2. Add the DNS records GCP provides:
   - **Type A** records pointing to GCP IPs
   - **Type AAAA** records for IPv6
3. Wait for DNS propagation (5 min – 48 hours)
4. GCP automatically provisions an SSL certificate (HTTPS)

### Option B: Cloud Load Balancer + Cloud CDN (more control)

For better performance and caching:

```bash
# Create a serverless NEG (Network Endpoint Group)
gcloud compute network-endpoint-groups create riscvml-neg \
  --region=europe-west1 \
  --network-endpoint-type=serverless \
  --cloud-run-service=riscvml-web

# Create backend service
gcloud compute backend-services create riscvml-backend \
  --global \
  --enable-cdn

# Add NEG to backend
gcloud compute backend-services add-backend riscvml-backend \
  --global \
  --network-endpoint-group=riscvml-neg \
  --network-endpoint-group-region=europe-west1

# Create URL map
gcloud compute url-maps create riscvml-urlmap \
  --default-service riscvml-backend

# Create SSL certificate
gcloud compute ssl-certificates create riscvml-cert \
  --domains=riscvml.org,www.riscvml.org

# Create HTTPS proxy
gcloud compute target-https-proxies create riscvml-https-proxy \
  --ssl-certificates=riscvml-cert \
  --url-map=riscvml-urlmap

# Create forwarding rule (public IP)
gcloud compute forwarding-rules create riscvml-https-rule \
  --global \
  --target-https-proxy=riscvml-https-proxy \
  --ports=443
```

Then point your domain's DNS A record to the forwarding rule's IP.

## Step 8: Set Up riscvml.com Redirect

In your domain registrar, set up a 301 redirect:
- `riscvml.com` → `https://riscvml.org`
- `www.riscvml.com` → `https://riscvml.org`

## Step 9: CI/CD (Optional — Auto-Deploy on Push)

### Using Cloud Build + GitHub

```bash
# Connect your GitHub repo
gcloud builds triggers create github \
  --repo-name=riscvml \
  --repo-owner=sdnscottie \
  --branch-pattern="^master$" \
  --build-config=riscvml_website_gcp/cloudbuild.yaml
```

Create `cloudbuild.yaml`:

```yaml
steps:
  - name: 'gcr.io/cloud-builders/docker'
    args: ['build', '-t', 'europe-west1-docker.pkg.dev/$PROJECT_ID/riscvml/riscvml-web', '-f', 'riscvml_website_gcp/Dockerfile', 'riscvml_website_gcp']
  - name: 'gcr.io/cloud-builders/docker'
    args: ['push', 'europe-west1-docker.pkg.dev/$PROJECT_ID/riscvml/riscvml-web']
  - name: 'gcr.io/google.com/cloudsdktool/cloud-sdk'
    args: ['gcloud', 'run', 'deploy', 'riscvml-web', '--image', 'europe-west1-docker.pkg.dev/$PROJECT_ID/riscvml/riscvml-web', '--region', 'europe-west1']
```

Now every push to master auto-deploys.

---

## Cost Estimate (Monthly)

| Resource | Free Tier | Expected Usage | Cost |
|----------|-----------|---------------|------|
| Cloud Run | 2M requests/mo | ~50K requests | **$0** |
| Cloud Build | 120 min/day | ~5 min/deploy | **$0** |
| Artifact Registry | 500MB | ~100MB image | **$0** |
| Domain mapping SSL | Free | Included | **$0** |
| **Total** | | | **~$0/month** |

Cloud Run scales to zero when idle — you only pay for actual requests.

---

## Quick Reference

```bash
# Redeploy after changes
gcloud run deploy riscvml-web --source . --region europe-west1

# View logs
gcloud run services logs read riscvml-web --region europe-west1

# View service details
gcloud run services describe riscvml-web --region europe-west1

# Delete service (if needed)
gcloud run services delete riscvml-web --region europe-west1
```
