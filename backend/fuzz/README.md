# Fuzz Testing

Этот каталог содержит fuzz-тесты для backend модуля, использующие `cargo-fuzz` и `libfuzzer-sys`.

## Структура

```
fuzz/
├── Cargo.toml              # Конфигурация fuzz-тестов
├── .gitignore             # Исключения для артефактов
├── README.md              # Этот файл
└── fuzz_targets/
    ├── orderbook.rs       # Fuzz логики ордербука
    ├── swap_engine.rs     # Fuzz логики свапа
    ├── api_handlers.rs    # Fuzz API-структур
    ├── pricing.rs         # Fuzz модуля ценообразования
    ├── types.rs           # Fuzz сериализации DTO
    ├── arbitrary_types.rs # Реализация Arbitrary для типов
    └── README.md          # Описание тестов
```

## Запуск fuzz-тестов

### Локально

```bash
# Установка cargo-fuzz
cargo install cargo-fuzz

# Запуск конкретного теста
cd backend/fuzz
cargo fuzz run orderbook -- -runs=1000

# Запуск всех тестов
cargo fuzz run orderbook -- -runs=1000
cargo fuzz run swap_engine -- -runs=1000
cargo fuzz run api_handlers -- -runs=1000
cargo fuzz run pricing -- -runs=1000
cargo fuzz run types -- -runs=1000
```

### Параметры

- `-runs=1000` - количество итераций
- `-max_len=4096` - максимальная длина входных данных
- `-timeout=300` - таймаут в секундах

## CI/CD

Fuzz-тесты автоматически запускаются:

1. **На каждый PR** - быстрые тесты (1000 итераций)
2. **На каждый push в main/develop** - быстрые тесты
3. **Ежедневно в 2:00 UTC** - быстрые тесты
4. **Ежедневно в 3:00 UTC** - глубокие тесты (10000 итераций)

## Анализ результатов

### Успешный запуск
```
Done 1000 runs in 0 second(s)
```

### Найдены проблемы
```
==12345==ERROR: AddressSanitizer: heap-buffer-overflow
```

### Покрытие
- `cov: 460` - количество покрытых строк кода
- `ft: 511` - количество уникальных путей выполнения
- `corpus: 28/99b` - количество уникальных тестовых случаев

## Добавление новых fuzz-тестов

1. Создайте новый файл в `fuzz_targets/`
2. Добавьте `[[bin]]` секцию в `Cargo.toml`
3. Используйте `#![no_main]` и `libfuzzer_sys::fuzz_target!`
4. Для внешних типов используйте newtype-обёртки

### Пример

```rust
#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Ваш fuzz-код здесь
});
```

## Best Practices

1. **Используйте Arbitrary** для генерации структур
2. **Обрабатывайте ошибки** - не паникуйте на некорректных данных
3. **Тестируйте граничные случаи** - нули, отрицательные числа, очень большие значения
4. **Добавляйте комментарии** - объясняйте что тестируется
5. **Используйте таймауты** - предотвращайте бесконечные циклы

## Troubleshooting

### Ошибка компиляции
```bash
cargo check
```

### Проблемы с зависимостями
```bash
cargo clean
cargo update
```

### Проблемы с LLVM
```bash
# Ubuntu/Debian
sudo apt-get install clang llvm-dev libclang-dev

# macOS
brew install llvm
```

## Контакты

При обнаружении проблем создавайте issue с:
- Воспроизводимым примером
- Логами fuzz-теста
- Информацией об окружении 