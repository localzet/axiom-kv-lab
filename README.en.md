# axiom-kv-lab v0.2.0

The self-evolution experiment now rejects **two different architectural failures** before promotion:

1. `volatile-v1` fails crash persistence;
2. `journaled-open-v2` persists state but fails authorization;
3. `journaled-cap-v3` satisfies the current model and is promoted.

Run `python modelcheck.py`. The experiment writes a machine-readable evolution history and promotion receipt.

## Авторство

Сопровождающий собственных изменений: **Ivan Zorin (localzet)** — <creator@localzet.com> · https://www.localzet.com. Copyright © 2026 Localzet Group. Исходное авторство и лицензии сторонних компонентов сохраняются. См. [AUTHORS](.github/AUTHORS.md).
