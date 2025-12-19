# Service Health Monitor & Alerting System 


## Overview

```
┌─────────────────┐    ┌──────────────────┐    ┌──────────────────┐
│   Config Loader │───▶│  Scheduler Engine│───▶│ Health Checkers  │
│   (TOML/YAML)   │    │   (Tokio Cron)   │    │ HTTP/TCP/ICMP    │
└─────────────────┘    └──────────────────┘    └─────────┬────────┘
                                                          │
┌─────────────────┐    ┌──────────────────┐              ▼
│   Web API       │◀───│   State Store    │◀─────[Check Results]
│   (Axum)        │    │   (Database)     │
└────────┬────────┘    └──────────────────┘
         │                       │
         ▼                       ▼
┌─────────────────┐    ┌──────────────────┐
│  Web Dashboard  │    │ Alerting Engine  │
│  (Optional UI)  │    │ (Rules → Notify) │
└─────────────────┘    └──────────────────┘

```

## What does this project solve ?

- Imagine you're the DevOps team at "CloudCorp". You have:

    5 microservices running

    3 databases

    2 external APIs you depend on

- Your problem: When something goes down at 2 AM, you want to know:

    What failed?

    When did it fail?

    How to get notified?

    What's the current status?

- Currently, someone has to manually check everything. This tool automates this process.