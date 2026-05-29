#!/usr/bin/env bash
# Creates a test git repository for AsciiDiff development.
# Usage: ./scripts/create-test-repo.sh [target-dir]

set -euo pipefail

TARGET="${1:-$(pwd)/test-repo}"

if [ -d "$TARGET/.git" ]; then
  echo "Test repo already exists at $TARGET — removing to recreate."
  rm -rf "$TARGET"
fi

mkdir -p "$TARGET"/{partials,chapters,modules/auth,modules/gateway}
cd "$TARGET"
git init
git config user.email "test@asciidiff.dev"
git config user.name "Test Author"

# ─── Main branch content ────────────────────────────────────────────────────

cat > partials/_header.adoc << 'EOF'
:toc: left
:toclevels: 3
:icons: font
:source-highlighter: rouge
:experimental:
:sectanchors:
EOF

cat > partials/_footer.adoc << 'EOF'

---

[.small]
Copyright (C) 2025 Example Corp. All rights reserved. +
Generated from branch `{git-branch}` at `{localdate}`.
EOF

cat > partials/_config.adoc << 'EOF'
=== Configuration Files

All configuration is managed via YAML files stored in the `config/` directory.

Environment-specific overrides are applied via `config/{env}.yaml`.

[source,yaml]
----
app:
  name: platform-core
  version: 1.4.0
  environment: ${ENV:-development}
----
EOF

cat > partials/_database.adoc << 'EOF'
=== Database Configuration

The platform uses PostgreSQL 15 for persistent storage and Redis 7 for caching.

[source,yaml]
----
database:
  host: ${DB_HOST:-localhost}
  port: 5432
  name: platform
  pool_size: 20
  ssl: require

cache:
  host: ${REDIS_HOST:-localhost}
  port: 6379
  ttl: 3600
----

WARNING: Never store database credentials in configuration files. Use environment variables or a secrets manager.
EOF

cat > chapters/overview.adoc << 'EOF'
== Overview

This document describes the core architectural components of the platform and how they interact with external services deployed on the cloud infrastructure.

NOTE: Refer to the deployment guide for infrastructure-specific configuration steps before proceeding.

The platform follows a microservices architecture with the following principles:

* Each service owns its data store
* Communication is asynchronous where possible
* All services expose health endpoints at `/healthz`
* Circuit breakers protect against cascade failures
EOF

cat > chapters/components.adoc << 'EOF'
== Components

=== API Gateway

The API Gateway handles all inbound traffic and routes requests to downstream services based on path matching rules defined in `gateway.yaml`.

.Gateway Request Flow
[plantuml]
----
@startuml
Client -> Gateway : HTTP Request
Gateway -> Auth : Validate Token
Auth --> Gateway : Token OK
Gateway -> Service : Forward Request
Service --> Gateway : Response
Gateway --> Client : HTTP Response
@enduml
----

=== Legacy Proxy

A thin reverse proxy used for backward compatibility with v1 clients. Routes `/api/v1/*` traffic to the deprecated service handlers.

CAUTION: The Legacy Proxy is scheduled for removal in v2.0. Begin migration planning now.

=== Auth Service

The authentication service handles:

. Token issuance (OAuth2 + OIDC)
. Token validation and introspection
. Session management
. Role-based access control (RBAC)

include::../modules/auth/overview.adoc[]
EOF

cat > chapters/deployment.adoc << 'EOF'
== Deployment Notes

Deployments are managed through the CI pipeline. Each service is containerized and orchestrated via the internal Kubernetes cluster running on AWS EKS.

* Staging deploys on every merge to `develop`
* Production releases require manual approval from a lead engineer
* Rollbacks are automated when error rate exceeds 5% over 5 minutes

=== Resource Requirements

|===
| Service | CPU Request | CPU Limit | Memory Request | Memory Limit

| API Gateway | 250m | 1000m | 256Mi | 1Gi
| Auth Service | 100m | 500m | 128Mi | 512Mi
| Cache Layer | 50m | 200m | 64Mi | 256Mi
| Legacy Proxy | 50m | 200m | 64Mi | 128Mi
|===

=== Environment Variables

All services read configuration from the environment. The following variables are required:

[cols="2,1,3"]
|===
| Variable | Required | Description

| `DATABASE_URL` | Yes | PostgreSQL connection string
| `REDIS_URL` | Yes | Redis connection string
| `JWT_SECRET` | Yes | Secret key for JWT signing
| `LOG_LEVEL` | No | Logging verbosity (default: `info`)
| `OTEL_ENDPOINT` | No | OpenTelemetry collector URL
|===
EOF

cat > modules/auth/overview.adoc << 'EOF'
==== Authentication Architecture

The auth module uses a layered approach:

[source]
----
┌─────────────────────────────────┐
│         API Gateway             │
├─────────────────────────────────┤
│     Token Validation Layer      │
├─────────────────────────────────┤
│      Session Store (Redis)      │
├─────────────────────────────────┤
│   Identity Provider (Postgres)  │
└─────────────────────────────────┘
----

Tokens are issued as JWTs with a 15-minute expiry. Refresh tokens are stored in Redis with a 7-day TTL.
EOF

cat > modules/gateway/routes.adoc << 'EOF'
==== Gateway Routing Rules

Routes are defined declaratively:

[source,yaml]
----
routes:
  - path: /api/v1/*
    service: legacy-proxy
    timeout: 10s
  - path: /api/v2/*
    service: api-gateway
    timeout: 30s
  - path: /auth/*
    service: auth-service
    timeout: 5s
----

Each route supports:

* Path-based matching with glob patterns
* Per-route timeout configuration
* Custom header injection
* Rate limiting overrides
EOF

cat > architecture-guide.adoc << 'EOF'
= System Architecture Guide
Engineering Team <engineering@example.com>

include::partials/_header.adoc[]

include::chapters/overview.adoc[]

include::chapters/components.adoc[]

== Configuration

include::partials/_config.adoc[]

include::partials/_database.adoc[]

== Service Matrix

|===
| Service | Port | Protocol | Owner

| API Gateway | 8080 | HTTP/2 | Platform
| Auth Service | 9001 | gRPC | Identity
| Cache Layer | 6379 | Redis | Infra
| Legacy Proxy | 8000 | HTTP/1.1 | Platform
|===

include::chapters/deployment.adoc[]

include::partials/_footer.adoc[]
EOF

cat > api-reference.adoc << 'EOF'
= API Reference
Engineering Team
:toc:
:source-highlighter: rouge

include::partials/_header.adoc[]

== Authentication

All API requests must include a Bearer token in the `Authorization` header.

[source,bash]
----
curl -H "Authorization: Bearer <token>" \
     https://api.example.com/v1/resources
----

== Endpoints

=== Resources

|===
| Method | Path | Description

| GET | /v1/resources | List all resources
| POST | /v1/resources | Create a resource
| GET | /v1/resources/:id | Get a single resource
| PUT | /v1/resources/:id | Replace a resource
| DELETE | /v1/resources/:id | Delete a resource
|===

=== Users

|===
| Method | Path | Description

| GET | /v1/users/me | Get current user profile
| PATCH | /v1/users/me | Update current user
| GET | /v1/users/:id | Get user by ID (admin)
|===

NOTE: Rate limiting is enforced at 100 requests/minute per token.

== Error Codes

|===
| Code | Meaning | Retry?

| 400 | Bad request — malformed JSON or missing required fields | No
| 401 | Unauthorized — token expired or invalid | No
| 403 | Forbidden — insufficient permissions | No
| 404 | Not found | No
| 429 | Rate limited | Yes (after `Retry-After` header delay)
| 500 | Internal error | Yes (with exponential backoff)
|===

== Pagination

All list endpoints support cursor-based pagination:

[source,bash]
----
GET /v1/resources?limit=50&cursor=eyJpZCI6MTAwfQ==
----

Response includes a `next_cursor` field when more results are available.

include::partials/_footer.adoc[]
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
  max_connections: 1000
  idle_timeout: 60s
----

== Supported Endpoints

The legacy proxy handles the following deprecated endpoints:

|===
| Endpoint | Replacement

| /api/v1/users | /v2/users
| /api/v1/resources | /v2/resources
| /api/v1/config | /v2/settings
|===

== Troubleshooting

* Ensure the proxy port is not blocked by the firewall
* Check logs at `/var/log/legacy-proxy/error.log`
* Restart with `systemctl restart legacy-proxy`
* If connections are dropping, increase `max_connections`
EOF

cat > runbook.adoc << 'EOF'
= Operations Runbook
SRE Team
:toc:
:icons: font

include::partials/_header.adoc[]

== Incident Response

=== High Error Rate

When the error rate exceeds 5% for more than 2 minutes:

. Check the Grafana dashboard for affected services
. Identify the failing endpoint via access logs
. If a single service is failing, check its pod logs:
+
[source,bash]
----
kubectl logs -l app=<service-name> --tail=100 -f
----

. If the issue is database-related, check connection pool exhaustion
. Escalate to the on-call engineer if not resolved within 10 minutes

=== Pod CrashLoopBackOff

. Check the pod events: `kubectl describe pod <name>`
. Check for OOMKilled: increase memory limits if needed
. Check for config errors: verify environment variables

=== Certificate Expiry

TIP: Certificates are auto-renewed 30 days before expiry via cert-manager.

If auto-renewal fails:

[source,bash]
----
# Force renewal
kubectl delete certificate <name> -n <namespace>
# cert-manager will recreate it automatically
----

== Scheduled Maintenance

|===
| Task | Frequency | Owner | Runbook

| Database vacuum | Weekly (Sun 03:00 UTC) | DBA | link:db-maintenance.adoc[]
| Certificate rotation | Monthly | SRE | Automated
| Dependency updates | Bi-weekly | Dev | PR-based
| Load testing | Before each release | QA | link:load-test.adoc[]
|===

include::partials/_footer.adoc[]
EOF

git add -A
git commit -m "docs: initial architecture documentation with includes"
git tag v1.4.0

# ─── feature/v2 branch ──────────────────────────────────────────────────────

git checkout -b feature/v2

cat > partials/_header.adoc << 'EOF'
:toc: left
:toclevels: 4
:icons: font
:source-highlighter: rouge
:experimental:
:sectanchors:
:last-update-label!:
EOF

cat > partials/_config.adoc << 'EOF'
=== Configuration Files

All configuration is managed via YAML files stored in the `config/` directory. In v2, configuration supports hot-reloading via a file watcher.

Environment-specific overrides are applied via `config/{env}.yaml`.

[source,yaml]
----
app:
  name: platform-core
  version: 2.0.0
  environment: ${ENV:-development}
  features:
    service_mesh: true
    hot_reload: true
----

TIP: Set `features.hot_reload: true` to enable runtime configuration updates without restarting services.
EOF

cat > partials/_database.adoc << 'EOF'
=== Database Configuration

The platform uses PostgreSQL 16 for persistent storage, Redis 7 for caching, and introduces ClickHouse for analytics.

[source,yaml]
----
database:
  host: ${DB_HOST:-localhost}
  port: 5432
  name: platform
  pool_size: 50
  ssl: require
  statement_timeout: 30s

cache:
  host: ${REDIS_HOST:-localhost}
  port: 6379
  ttl: 3600
  cluster: true

analytics:
  host: ${CLICKHOUSE_HOST:-localhost}
  port: 9000
  database: platform_events
----

WARNING: Never store database credentials in configuration files. Use environment variables or a secrets manager.

NOTE: Pool size increased from 20 to 50 to support service mesh sidecar connections.
EOF

cat > partials/_observability.adoc << 'EOF'
=== Observability Stack

All services emit structured logs, metrics, and traces:

.Observability Architecture
[cols="1,2,2"]
|===
| Signal | Tool | Retention

| Logs | Loki + Grafana | 30 days
| Metrics | Prometheus + Thanos | 90 days
| Traces | Jaeger + Tempo | 7 days
|===

[source,yaml]
----
observability:
  logs:
    format: json
    level: ${LOG_LEVEL:-info}
  metrics:
    port: 9090
    path: /metrics
  traces:
    endpoint: ${OTEL_ENDPOINT}
    sample_rate: 0.1
----
EOF

cat > chapters/overview.adoc << 'EOF'
== Overview

This document describes the core architectural components of the platform and how they interact with external services via a fully managed service mesh on AWS EKS.

NOTE: Refer to the deployment guide for infrastructure-specific configuration steps before proceeding.

The platform follows a microservices architecture with the following principles:

* Each service owns its data store
* Communication is asynchronous where possible (via NATS JetStream)
* All services expose health endpoints at `/healthz` and `/readyz`
* Circuit breakers protect against cascade failures
* mTLS is enforced on all inter-service communication
* All services are observable via OpenTelemetry

=== What Changed in v2

[cols="1,1"]
|===
| v1 | v2

| HTTP-based routing | Service mesh (Envoy)
| Manual scaling | HPA + KEDA
| Legacy proxy for v1 compat | Removed (migration complete)
| Single-region | Multi-region active-active
| PostgreSQL 15 | PostgreSQL 16 + ClickHouse
|===
EOF

cat > chapters/components.adoc << 'EOF'
== Components

=== API Gateway

The API Gateway handles all inbound and outbound traffic, applying rate limiting, authentication middleware, and dynamic routing rules. Replaces static path config with policy-based routing.

.Gateway Request Flow (v2)
[plantuml]
----
@startuml
Client -> "Envoy Sidecar" : HTTP/2 Request
"Envoy Sidecar" -> Gateway : mTLS
Gateway -> Auth : Validate Token (gRPC)
Auth --> Gateway : Token OK + Claims
Gateway -> Service : Forward + Context
Service --> Gateway : Response
Gateway --> "Envoy Sidecar" : Response
"Envoy Sidecar" --> Client : HTTP/2 Response
@enduml
----

=== Service Mesh

All inter-service communication is now routed through an Envoy-based service mesh with mTLS enforced on every connection. Observability is provided by Jaeger tracing and Prometheus metrics.

WARNING: The Legacy Proxy component has been removed. All v1 clients must migrate to the v2 API before upgrading.

Features of the service mesh:

* Automatic mTLS between all services
* Traffic splitting for canary deployments
* Retry policies with jitter
* Circuit breaking with configurable thresholds
* Request-level observability

include::../modules/gateway/routes.adoc[]

=== Auth Service

The authentication service handles:

. Token issuance (OAuth2 + OIDC)
. Token validation and introspection
. Session management
. Role-based access control (RBAC)
. Multi-factor authentication (new in v2)
. API key management (new in v2)

include::../modules/auth/overview.adoc[]
EOF

cat > chapters/deployment.adoc << 'EOF'
== Deployment Notes

Deployments are managed through the CI pipeline. Each service is containerized and orchestrated via the internal Kubernetes cluster running on AWS EKS.

* Staging deploys on every merge to `develop`
* Production releases require manual approval from a lead engineer
* Rollbacks are automated when error rate exceeds 2% over 5 minutes
* Service mesh health checks run every 10s with automatic circuit-breaking
* Canary deployments shift 10% traffic, then 50%, then 100%

=== Resource Requirements

|===
| Service | CPU Request | CPU Limit | Memory Request | Memory Limit | Replicas

| API Gateway | 500m | 2000m | 512Mi | 2Gi | 3
| Auth Service | 250m | 1000m | 256Mi | 1Gi | 3
| Cache Layer | 100m | 500m | 128Mi | 512Mi | 3 (cluster)
| Event Bus | 250m | 1000m | 512Mi | 2Gi | 3
| Analytics | 500m | 2000m | 1Gi | 4Gi | 2
|===

=== Environment Variables

All services read configuration from the environment. The following variables are required:

[cols="2,1,3"]
|===
| Variable | Required | Description

| `DATABASE_URL` | Yes | PostgreSQL connection string
| `REDIS_URL` | Yes | Redis connection string (cluster mode)
| `JWT_SECRET` | Yes | Secret key for JWT signing
| `LOG_LEVEL` | No | Logging verbosity (default: `info`)
| `OTEL_ENDPOINT` | Yes | OpenTelemetry collector URL
| `NATS_URL` | Yes | NATS JetStream connection
| `CLICKHOUSE_URL` | No | ClickHouse connection (analytics only)
| `MESH_CONTROL_PLANE` | Yes | Service mesh control plane address
|===

=== Deployment Checklist

. [ ] All e2e tests pass on staging
. [ ] Security scan reports no critical CVEs
. [ ] Database migrations are backward-compatible
. [ ] Feature flags configured for gradual rollout
. [ ] Runbook updated for new failure modes
. [ ] On-call team briefed on changes
EOF

cat > modules/auth/overview.adoc << 'EOF'
==== Authentication Architecture

The auth module uses a layered approach with multi-factor support:

[source]
----
┌─────────────────────────────────┐
│         API Gateway             │
├─────────────────────────────────┤
│     Token Validation Layer      │
├─────────────────────────────────┤
│       MFA Challenge Layer       │  ← New in v2
├─────────────────────────────────┤
│      Session Store (Redis)      │
├─────────────────────────────────┤
│   Identity Provider (Postgres)  │
└─────────────────────────────────┘
----

Tokens are issued as JWTs with a 15-minute expiry. Refresh tokens are stored in Redis with a 7-day TTL.

.Supported MFA Methods
|===
| Method | Priority | Fallback

| TOTP (Authenticator App) | Primary | SMS
| WebAuthn / FIDO2 | Primary | TOTP
| SMS OTP | Secondary | Email
| Email OTP | Tertiary | Support ticket
|===
EOF

cat > modules/gateway/routes.adoc << 'EOF'
==== Gateway Routing Rules

Routes are now policy-based with traffic splitting support:

[source,yaml]
----
routes:
  - path: /api/v2/*
    service: api-gateway
    timeout: 30s
    retry:
      attempts: 3
      backoff: exponential
    circuit_breaker:
      threshold: 5
      timeout: 30s
  - path: /auth/*
    service: auth-service
    timeout: 5s
    rate_limit:
      requests: 20
      window: 60s
  - path: /events/*
    service: event-bus
    timeout: 60s

traffic_splitting:
  - service: api-gateway
    canary:
      weight: 10
      header: x-canary=true
----

Each route supports:

* Path-based matching with glob patterns
* Per-route timeout and retry configuration
* Custom header injection and transformation
* Rate limiting overrides per route
* Circuit breaker with configurable thresholds
* Traffic splitting for canary deployments
EOF

cat > architecture-guide.adoc << 'EOF'
= System Architecture Guide
Engineering Team <engineering@example.com>

include::partials/_header.adoc[]

include::chapters/overview.adoc[]

include::chapters/components.adoc[]

== Configuration

include::partials/_config.adoc[]

include::partials/_database.adoc[]

include::partials/_observability.adoc[]

== Service Matrix

|===
| Service | Port | Protocol | Owner | Health

| API Gateway | 8080 | HTTP/2 | Platform | healthy
| Auth Service | 9001 | gRPC | Identity | healthy
| Cache Layer | 6379 | Redis | Infra | degraded
| Event Bus | 4222 | NATS | Platform | healthy
| Analytics | 9000 | HTTP | Data | healthy
|===

include::chapters/deployment.adoc[]

include::partials/_footer.adoc[]
EOF

cat > api-reference.adoc << 'EOF'
= API Reference
Engineering Team
:toc:
:source-highlighter: rouge

include::partials/_header.adoc[]

== Authentication

All API requests must include a Bearer token in the `Authorization` header.

[source,bash]
----
curl -H "Authorization: Bearer <token>" \
     https://api.example.com/v2/resources
----

== Endpoints

=== Resources

|===
| Method | Path | Description

| GET | /v2/resources | List all resources (paginated)
| POST | /v2/resources | Create a resource
| GET | /v2/resources/:id | Get a single resource
| PUT | /v2/resources/:id | Replace a resource
| PATCH | /v2/resources/:id | Partial update
| DELETE | /v2/resources/:id | Delete a resource (soft-delete)
|===

=== Users

|===
| Method | Path | Description

| GET | /v2/users/me | Get current user profile
| PATCH | /v2/users/me | Update current user
| GET | /v2/users/:id | Get user by ID (admin)
| POST | /v2/users/:id/mfa | Enable MFA for user
| DELETE | /v2/users/:id/mfa | Disable MFA for user
|===

=== Events (new in v2)

|===
| Method | Path | Description

| GET | /v2/events | List events (cursor-paginated)
| POST | /v2/events | Publish an event
| GET | /v2/events/stream | SSE stream of real-time events
|===

NOTE: Rate limiting is enforced at 500 requests/minute per token. Burst allowance of 50 requests.

== Error Codes

|===
| Code | Meaning | Retry?

| 400 | Bad request — malformed JSON or missing required fields | No
| 401 | Unauthorized — token expired or invalid | No
| 403 | Forbidden — insufficient permissions | No
| 404 | Not found | No
| 409 | Conflict — resource version mismatch | Yes (re-fetch and retry)
| 422 | Unprocessable — validation failed | No
| 429 | Rate limited | Yes (after `Retry-After` header delay)
| 500 | Internal error | Yes (with exponential backoff)
| 503 | Service unavailable (circuit breaker open) | Yes (after 30s)
|===

== Pagination

All list endpoints support cursor-based pagination:

[source,bash]
----
GET /v2/resources?limit=50&cursor=eyJpZCI6MTAwfQ==
----

Response includes a `next_cursor` field when more results are available.

== Webhooks (new in v2)

Subscribe to resource events via webhooks:

[source,bash]
----
POST /v2/webhooks
Content-Type: application/json

{
  "url": "https://your-app.com/hooks/resources",
  "events": ["resource.created", "resource.updated", "resource.deleted"],
  "secret": "whsec_..."
}
----

include::partials/_footer.adoc[]
EOF

cat > deployment-guide.adoc << 'EOF'
= Deployment Guide
Infrastructure Team
:toc:
:icons: font

include::partials/_header.adoc[]

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

== Canary Deployment

New in v2 — gradual traffic shifting:

[source,bash]
----
# Start canary (10% traffic)
make canary-start ENV=production TAG=v2.0.0

# Promote to 50%
make canary-promote ENV=production

# Full rollout
make canary-finish ENV=production

# Abort canary
make canary-abort ENV=production
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
* Canary metrics: `meshctl canary status`

include::partials/_footer.adoc[]
EOF

cat > runbook.adoc << 'EOF'
= Operations Runbook
SRE Team
:toc:
:icons: font

include::partials/_header.adoc[]

== Incident Response

=== High Error Rate

When the error rate exceeds 2% for more than 2 minutes:

. Check the Grafana dashboard for affected services
. Check service mesh circuit breaker status: `meshctl cb status`
. Identify the failing endpoint via access logs
. If a single service is failing, check its pod logs:
+
[source,bash]
----
kubectl logs -l app=<service-name> --tail=100 -f
----

. If the issue is database-related, check connection pool exhaustion
. If circuit breaker is open, check downstream service health
. Escalate to the on-call engineer if not resolved within 10 minutes

=== Pod CrashLoopBackOff

. Check the pod events: `kubectl describe pod <name>`
. Check for OOMKilled: increase memory limits if needed
. Check for config errors: verify environment variables
. Check mesh sidecar logs: `kubectl logs <pod> -c envoy-sidecar`

=== Certificate Expiry

TIP: Certificates are auto-renewed 30 days before expiry via cert-manager.

If auto-renewal fails:

[source,bash]
----
# Force renewal
kubectl delete certificate <name> -n <namespace>
# cert-manager will recreate it automatically
----

=== Service Mesh Partition

When services cannot communicate through the mesh:

. Verify control plane health: `meshctl status`
. Check mTLS certificate validity: `meshctl proxy-config secret <pod>`
. Restart affected sidecars: `kubectl rollout restart deployment/<name>`
. If cluster-wide, restart the mesh control plane

== Scheduled Maintenance

|===
| Task | Frequency | Owner | Runbook

| Database vacuum | Weekly (Sun 03:00 UTC) | DBA | link:db-maintenance.adoc[]
| Certificate rotation | Monthly | SRE | Automated
| Dependency updates | Bi-weekly | Dev | PR-based
| Load testing | Before each release | QA | link:load-test.adoc[]
| Mesh control plane upgrade | Quarterly | SRE | link:mesh-upgrade.adoc[]
| ClickHouse compaction | Weekly (Sat 04:00 UTC) | Data | Automated
|===

include::partials/_footer.adoc[]
EOF

rm -f legacy-setup.adoc

git add -A
git commit -m "docs: v2 architecture overhaul

- Replace legacy proxy with service mesh (Envoy)
- Add observability stack (Loki, Prometheus, Jaeger)
- Add ClickHouse for analytics
- Add MFA support to auth module
- Add canary deployment workflow
- Add events API and webhooks
- Expand routing with traffic splitting
- Remove legacy-setup.adoc (deprecated)"

git tag v2.0.0

# ─── hotfix/auth-fix branch (off main) ──────────────────────────────────────

git checkout main
git checkout -b hotfix/auth-fix

# Small targeted change on main — only touch auth module
cat > modules/auth/overview.adoc << 'EOF'
==== Authentication Architecture

The auth module uses a layered approach:

[source]
----
┌─────────────────────────────────┐
│         API Gateway             │
├─────────────────────────────────┤
│     Token Validation Layer      │
├─────────────────────────────────┤
│      Session Store (Redis)      │
├─────────────────────────────────┤
│   Identity Provider (Postgres)  │
└─────────────────────────────────┘
----

Tokens are issued as JWTs with a 15-minute expiry. Refresh tokens are stored in Redis with a 7-day TTL.

IMPORTANT: Token validation now checks the `aud` (audience) claim. Tokens issued before v1.4.1 without an audience claim will be rejected. Re-issue tokens if users report 401 errors after the upgrade.

.Token Validation Flow
. Extract Bearer token from `Authorization` header
. Decode JWT header, verify algorithm is `ES256`
. Validate signature against the current signing key
. Check `exp` claim (reject if expired)
. Check `aud` claim (reject if mismatched)
. Check `iss` claim (must match configured issuer)
. Return decoded claims to the gateway
EOF

git add -A
git commit -m "fix(auth): document audience claim validation requirement"

# ─── Back to main ───────────────────────────────────────────────────────────

git checkout main

echo ""
echo "Test repo created at: $TARGET"
echo "  Branches: main, feature/v2, hotfix/auth-fix"
echo "  Tags: v1.4.0, v2.0.0"
echo ""
echo "  Suggested comparisons:"
echo "    main ↔ feature/v2        (major overhaul, new files, deleted files)"
echo "    main ↔ hotfix/auth-fix   (small targeted fix, single file)"
echo "    v1.4.0 ↔ v2.0.0         (tag-to-tag comparison)"
