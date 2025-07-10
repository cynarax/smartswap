# Changelog

## [0.1.0] — 2024-06-XX

### Release Notes
- Первый публичный релиз монорепозитория: smartswap_core, smartswap_backend
- Полная поддержка workspace, clippy clean, тесты, fuzzing, CI-инфраструктура

### Breaking Changes
- Переименование core → smartswap_core, backend → smartswap_backend
- Все импорты и зависимости обновлены на новые имена

### Migration
- Для обновления: заменить все импорты core:: на smartswap_core::, backend:: на smartswap_backend::
- Проверить зависимости в Cargo.toml 