# Security Policy

## Supported Versions
Only `main` is supported. Use the latest commit.

## Reporting a Vulnerability
Do NOT open a public issue for a security bug.

Email the maintainer via a GitHub issue titled `SECURITY: private disclosure requested` with no details, or use GitHub's private vulnerability reporting on this repo (Security > Report a vulnerability).

We will acknowledge within 72 hours, and aim to fix + disclose within 14 days.

## Scope
This repo is hygienic by design:
- No real TEE quote generation, no real zkVM STARK proof, no real on-chain payment.
- `starkVerified` is always `false` in this repo. Escrow stays `HELD` unless on-chain proof is supplied.
- TEE verification is fail-closed: missing or bad proof = no commit.
- Never paste real secrets, API keys, or private keys into issues, PRs, or commits.

## Safe Harbor
Good-faith security research is welcome. Do not exfiltrate data, do not DoS, do not break the law.
