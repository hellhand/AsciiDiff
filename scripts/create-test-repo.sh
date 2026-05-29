#!/usr/bin/env bash
# Creates a test git repository for AsciiDiff development.
# Usage: ./scripts/create-test-repo.sh [target-dir]

set -euo pipefail

TARGET="${1:-$(pwd)/test-repo}"

if [ -d "$TARGET/.git" ]; then
  echo "Test repo already exists at $TARGET"
  exit 0
fi

rm -rf "$TARGET"
mkdir -p "$TARGET/partials"
cd "$TARGET"
git init

# ─── Main branch content ────────────────────────────────────────────────────

cat > partials/_header.adoc << 'EOF'
:toc:
:toclevels: 3
:icons: font
:source-highlighter: rouge
EOF

cat > partials/_config.adoc << 'EOF'
=== Configuration Files

All configuration is managed via YAML files stored in the `config/` directory.

Environment-specific overrides are applied via `config/{env}.yaml`.
EOF

cat > architecture-guide.adoc << 'EOF'
= System Architecture Guide
Engineering Team <engineering@example.com>
:toc:
:toclevels: 3
:icons: font
:source-highlighter: rouge

include::partials/_header.adoc[]

== Overview

This document describes the core architectural components of the platform and how they interact with external services deployed on the cloud infrastructure.

NOTE: Refer to the deployment guide for infrastructure-specific configuration steps before proceeding.

== Components

=== API Gateway

The API Gateway handles all inbound traffic and routes requests to downstream services based on path matching rules defined in `gateway.yaml`.

=== Legacy Proxy

A thin reverse proxy used for backward compatibility with v1 clients. Routes `/api/v1/*` traffic to the deprecated service handlers.

== Configuration

include::partials/_config.adoc[]

[source,yaml]
----
gateway:
  port: 8080
  timeout: 30s
  retry_policy: exponential
----

== Service Matrix

|===
| Service | Port | Protocol | Owner

| API Gateway | 8080 | HTTP/2 | Platform
| Auth Service | 9001 | gRPC | Identity
| Cache Layer | 6379 | Redis | Infra
| Legacy Proxy | 8000 | HTTP/1.1 | Platform
|===

== Deployment Notes

Deployments are managed through the CI pipeline. Each service is containerized and orchestrated via the internal Kubernetes cluster running on AWS EKS.

* Staging deploys on every merge to `develop`
* Production releases require manual approval from a lead engineer
* Rollbacks are automated when error rate exceeds 5% over 5 minutes
EOF

cat > api-reference.adoc << 'EOF'
= API Reference
Engineering Team
:toc:
:source-highlighter: rouge

== Authentication

All API requests must include a Bearer token in the `Authorization` header.

[source,bash]
----
curl -H "Authorization: Bearer <token>" \
     https://api.example.com/v1/resources
----

== Endpoints

|===
| Method | Path | Description

| GET | /v1/resources | List all resources
| POST | /v1/resources | Create a resource
| DELETE | /v1/resources/:id | Delete a resource
|===

NOTE: Rate limiting is enforced at 100 requests/minute per token.

== Error Codes

|===
| Code | Meaning | Retry?

| 400 | Bad request | No
| 401 | Unauthorized | No
| 429 | Rate limited | Yes (after delay)
| 500 | Internal error | Yes
|===
EOF

cat > legacy-setup.adoc << 'EOF'
= Legacy Setup Guide
Platform Team
:toc:

== Installation

Install the legacy proxy component before configuring the gateway. This component provides backward compatibility for v1 API clients.

WARNING: This guide applies to v1.x only. See the main architecture guide for v2.

== Configuration

[source,yaml]
----
proxy:
  listen: 0.0.0.0:8000
  upstream: http://localhost:8080
  rewrite_host: true
----

== Troubleshooting

* Ensure the proxy port is not blocked by the firewall
* Check logs at `/var/log/legacy-proxy/error.log`
* Restart with `systemctl restart legacy-proxy`
EOF

git add -A
git commit -m "docs: initial architecture documentation"
git tag v1.4.0

# ─── feature/v2 branch ──────────────────────────────────────────────────────

git checkout -b feature/v2

cat > partials/_header.adoc << 'EOF'
:toc:
:toclevels: 4
:icons: font
:source-highlighter: rouge
EOF

cat > architecture-guide.adoc << 'EOF'
= System Architecture Guide
Engineering Team <engineering@example.com>
:toc:
:toclevels: 3
:icons: font
:source-highlighter: rouge

include::partials/_header.adoc[]

== Overview

This document describes the core architectural components of the platform and how they interact with external services via a fully managed service mesh on AWS EKS.

NOTE: Refer to the deployment guide for infrastructure-specific configuration steps before proceeding.

== Components

=== API Gateway

The API Gateway handles all inbound and outbound traffic, applying rate limiting, authentication middleware, and dynamic routing rules. Replaces static path config with policy-based routing.

=== Service Mesh

All inter-service communication is now routed through an Envoy-based service mesh with mTLS enforced on every connection. Observability is provided by Jaeger tracing and Prometheus metrics.

WARNING: The Legacy Proxy component has been removed. All v1 clients must migrate to the v2 API before upgrading.

== Configuration

include::partials/_config.adoc[]

[source,yaml]
----
gateway:
  port: 8080
  timeout: 60s
  retry_policy: exponential
  mesh: enabled
  mtls: strict
----

== Service Matrix

|===
| Service | Port | Protocol | Owner | Health

| API Gateway | 8080 | HTTP/2 | Platform | healthy
| Auth Service | 9001 | gRPC | Identity | healthy
| Cache Layer | 6379 | Redis | Infra | degraded
|===

== Deployment Notes

Deployments are managed through the CI pipeline. Each service is containerized and orchestrated via the internal Kubernetes cluster running on AWS EKS.

* Staging deploys on every merge to `develop`
* Production releases require manual approval from a lead engineer
* Rollbacks are automated when error rate exceeds 2% over 5 minutes
* Service mesh health checks run every 10s with automatic circuit-breaking
EOF

cat > api-reference.adoc << 'EOF'
= API Reference
Engineering Team
:toc:
:source-highlighter: rouge

== Authentication

All API requests must include a Bearer token in the `Authorization` header.

[source,bash]
----
curl -H "Authorization: Bearer <token>" \
     https://api.example.com/v2/resources
----

== Endpoints

|===
| Method | Path | Description

| GET | /v2/resources | List all resources
| POST | /v2/resources | Create a resource
| DELETE | /v2/resources/:id | Delete a resource
| PATCH | /v2/resources/:id | Partial update
|===

NOTE: Rate limiting is enforced at 500 requests/minute per token.

== Error Codes

|===
| Code | Meaning | Retry?

| 400 | Bad request | No
| 401 | Unauthorized | No
| 403 | Forbidden | No
| 429 | Rate limited | Yes (after delay)
| 500 | Internal error | Yes
| 503 | Service unavailable | Yes (circuit breaker)
|===
EOF

cat > deployment-guide.adoc << 'EOF'
= Deployment Guide
Infrastructure Team
:toc:
:icons: font

== Prerequisites

Before deploying, ensure you have access to the AWS account and the required IAM roles are configured.

TIP: Run `make preflight` to validate all prerequisites automatically.

== Deploy to Staging

[source,bash]
----
# Deploy to staging environment
make deploy ENV=staging TAG=$(git rev-parse --short HEAD)
----

== Deploy to Production

IMPORTANT: Production deployments require a PR review and passing CI on `main`.

[source,bash]
----
make deploy ENV=production TAG=v2.0.0 APPROVE=yes
----

== Rollback

In case of a failed deployment:

[source,bash]
----
make rollback ENV=production STEPS=1
----

== Monitoring

After deployment, verify health via:

* Grafana dashboard: https://monitoring.internal/deployments
* Service mesh status: `meshctl status --all`
* Error rate alert threshold: 2% over 5 minutes
EOF

rm -f legacy-setup.adoc

git add -A
git commit -m "docs: v2 architecture overhaul"

# ─── Back to main ───────────────────────────────────────────────────────────

git checkout main

echo ""
echo "✓ Test repo created at: $TARGET"
echo "  Branches: main, feature/v2"
echo "  Tags: v1.4.0"
echo ""
echo "  To test AsciiDiff, open this folder in the app and compare:"
echo "    main ↔ feature/v2"
