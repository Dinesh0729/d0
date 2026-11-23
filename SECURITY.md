# Security Policy

## Reporting a Vulnerability

The D0 project takes security seriously. Because D0 is specifically engineered with defensive cybersecurity principles, any security vulnerability in the language specification, lexer, parser, semantic analyzer, or standard libraries is addressed promptly.

### How to Report
Please do **NOT** report security vulnerabilities through public GitHub issues.

Instead, please send an email to the project maintainers:
- **Maintainer**: Dinesh
- **Email**: cdineshkumar42@gmail.com
- **Repository**: [https://github.com/Dinesh0729/d0](https://github.com/Dinesh0729/d0)

Please include:
1. Description of the issue / vulnerability.
2. Steps to reproduce the issue (including sample `.d0` code or parser input).
3. Potential security impact.

### Security Principles of D0
- **Safe by Default**: Memory safety and strict boundary checks.
- **Explicit Capabilities**: Sensitive system, process, or network testing features require user authorization.
- **Defensive Focus**: D0 standard libraries do not include credential theft, stealth persistence, or unauthorized scanning mechanisms.
