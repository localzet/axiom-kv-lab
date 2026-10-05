# axiom-kv-lab v0.2.0

Эксперимент self-evolution теперь отвергает **два разных архитектурных дефекта** до promotion:

1. `volatile-v1` нарушает сохранность после crash;
2. `journaled-open-v2` сохраняет состояние, но нарушает авторизацию;
3. `journaled-cap-v3` удовлетворяет текущей модели и допускается к promotion.

Запуск: `python modelcheck.py`. Эксперимент записывает машиночитаемую историю эволюции и promotion receipt.

## Авторство

Сопровождающий собственных изменений: **Ivan Zorin (localzet)** — <creator@localzet.com> · https://www.localzet.com. Copyright © 2026 Localzet Group. Исходное авторство и лицензии сторонних компонентов сохраняются. См. [AUTHORS](.github/AUTHORS.md).
