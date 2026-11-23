# D0 Security Model 0.1

D0 is intended to support legitimate security engineering, defensive analysis and authorized testing.

## Principles

- Safe-by-default APIs.
- Explicit capabilities for sensitive operations.
- User-visible authorization scope for network testing.
- No stealth, persistence, credential theft or unauthorized access features in the standard library.
- Sandboxed execution for untrusted code.
- Resource limits for CPU, memory, file access and network access.
- Auditable logs for privileged actions.
- Cryptography through established, maintained libraries; do not invent cryptographic algorithms.
- Dependency pinning, integrity checks and vulnerability review for packages.

## Capability model (planned)

A program should request narrowly scoped permissions, such as:
- read a specific directory;
- write to a designated output directory;
- connect to an explicitly allowed host and port;
- perform an audit against a declared, authorized target range.

The runtime or user should grant permissions explicitly. A source-code string claiming authorization is not sufficient by itself.

## Threat model

The runtime must consider malicious source programs, malformed bytecode, hostile input files, unsafe package dependencies and accidental resource exhaustion.

This document defines design intent, not a completed security guarantee. The implementation must be independently tested and reviewed.
