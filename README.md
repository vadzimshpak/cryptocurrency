# Cryptocurrency

Учебная криптовалюта на Rust: блокчейн с Proof of Work, кошельки на Ed25519 и HTTP API на Axum.

## Возможности

- Цепочка блоков с SHA-256 и Proof of Work (сложность: хеш заканчивается на `0000`)
- Транзакции с подписью Ed25519 (публичный ключ = адрес кошелька, Base58)
- Мемпул (`current_transactions`) до включения в блок при майнинге
- HTTP API: цепочка, баланс, отправка транзакций, faucet, майнинг
- Genesis-блок с системным балансом `1_000_000` у адреса `core0000`

## Стек

| Компонент | Библиотека |
|-----------|------------|
| HTTP | Axum + Tokio |
| Криптография | ed25519-dalek, sha2 |
| Кодирование | bs58, hex |
| Сериализация | serde, serde_json |

## Требования

- [Rust](https://rustup.rs/) (edition 2024)
- Cargo

## Запуск

```bash
cargo run
```

Сервер слушает `http://127.0.0.1:5000`.

Тесты:

```bash
cargo test
```

## Архитектура

```
src/
├── main.rs                 # Точка входа, роутинг Axum
├── blockchain/
│   ├── core.rs             # BlockChain: блоки, мемпул, транзакции
│   ├── block.rs            # Block + SHA-256 hash
│   ├── transaction.rs      # Подпись и проверка Ed25519
│   ├── wallet.rs           # Генерация пары ключей
│   └── core_modules/
│       ├── balance.rs      # Подсчёт баланса по цепочке
│       └── pow_transaction.rs  # Proof of Work
└── network/
    ├── chain.rs            # GET /chain
    ├── transaction.rs      # GET /transactions/*
    ├── faucet.rs           # GET /faucet
    ├── mine.rs             # GET /mine
    └── types.rs            # AppState (Arc<Mutex<BlockChain>>)
```

Состояние блокчейна хранится в памяти (`Arc<Mutex<BlockChain>>`) — после перезапуска данные теряются.

## Модель данных

### Block

| Поле | Тип | Описание |
|------|-----|----------|
| `index` | `usize` | Номер блока (с 1) |
| `timestamp` | `f64` | Unix-время создания |
| `transactions` | `Vec<Transaction>` | Транзакции блока |
| `proof` | `i64` | Nonce Proof of Work |
| `previous_hash` | `String` | SHA-256 предыдущего блока |

### Transaction

| Поле | Тип | Описание |
|------|-----|----------|
| `sender` | `String` | Адрес отправителя (публичный ключ Base58) |
| `recipient` | `String` | Адрес получателя |
| `amount` | `u64` | Сумма |
| `timestamp` | `f64` | Время создания |
| `signature` | `String` | Подпись Base58 (для системных — `"inner_sys"`) |

Сообщение для подписи: `{sender}:{recipient}:{amount}`.

### Wallet

Генерируется локально (`Wallet::new()`): пара Ed25519 в Base58. Публичный ключ — адрес в сети.

## Как устроен поток

1. При старте создаётся genesis-блок и системная транзакция `core0000 → "0"` на `1_000_000`.
2. Faucet добавляет в мемпул перевод `core0000 → address` на `100` (без подписи пользователя).
3. Пользовательская транзакция проверяет баланс и Ed25519-подпись, затем попадает в мемпул.
4. `/mine` ищет proof, создаёт новый блок из мемпула и очищает мемпул.
5. Баланс считается только по подтверждённым блокам (мемпул не учитывается).

## HTTP API

Базовый URL: `http://127.0.0.1:5000`

### `GET /`

Проверка живости. Ответ: `"Hello, world!"`.

### `GET /chain`

Текущее состояние блокчейна: массив блоков и мемпул.

```bash
curl http://127.0.0.1:5000/chain
```

### `GET /transactions/balance?address={address}`

Баланс адреса по цепочке.

```bash
curl "http://127.0.0.1:5000/transactions/balance?address=YOUR_PUBLIC_KEY"
```

Ответ:

```json
{ "balance": 100 }
```

### `GET /faucet?address={address}`

Начисляет `100` монет на адрес (системная транзакция от `core0000`). Попадает в мемпул — нужен майнинг.

```bash
curl "http://127.0.0.1:5000/faucet?address=YOUR_PUBLIC_KEY"
```

### `GET /transactions/send?from=&to=&signature=&amount=`

Отправка подписанной транзакции.

| Параметр | Описание |
|----------|----------|
| `from` | Публичный ключ отправителя (Base58) |
| `to` | Адрес получателя |
| `signature` | Подпись `from:to:amount` приватным ключом (Base58) |
| `amount` | Сумма (`u64`) |

Ошибки (`400`): недостаточно средств или неверная подпись.

```bash
curl "http://127.0.0.1:5000/transactions/send?from=SENDER&to=RECIPIENT&signature=SIG&amount=10"
```

### `GET /mine`

Майнинг: Proof of Work + новый блок из мемпула.

```bash
curl http://127.0.0.1:5000/mine
```

## Пример сценария

1. Сгенерировать кошелёк в коде/тестах через `Wallet::new()` (получить `public_key` и `private_key`).
2. Запросить faucet на публичный ключ.
3. Вызвать `/mine`, чтобы зачислить монеты в цепочку.
4. Проверить баланс через `/transactions/balance`.
5. Подписать транзакцию сообщением `{from}:{to}:{amount}` и отправить через `/transactions/send`.
6. Снова вызвать `/mine`.

Подпись в Rust (внутри проекта / в тестах):

```rust
use crate::blockchain::transaction::Transaction;
use crate::blockchain::wallet::Wallet;

let wallet = Wallet::new();
let mut tx = Transaction {
    sender: wallet.public_key.clone(),
    recipient: "RECIPIENT_PUBLIC_KEY".into(),
    amount: 10,
    timestamp: 0.0,
    signature: String::new(),
};
tx.signature = tx.sign(wallet.private_key.clone());
```

## Proof of Work

Ищется `proof`, такой что:

```text
SHA256("{last_proof}{proof}")` заканчивается на `"0000"
```

Сложность зафиксирована в коде (`valid_proof`).

## Ограничения (учебный проект)

- Нет P2P и консенсуса между узлами
- Состояние только в RAM
- Нет награды майнеру
- Faucet и `inner_transaction` без криптографической проверки
- Баланс не учитывает мемпул
- Эндпоинты используют GET даже для изменяющих операций (сделано чтобы все можно было провернуть в браузере)

## Лицензия

Учебный проект, лицензия не указана.
