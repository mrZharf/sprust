# Sprust 🦀

کتابخانه‌ای برای Rust جهت کار با **Bot API سروش پلاس**.

> 🚧 Sprust در حال توسعه است و هنوز در مرحله‌ی اولیه‌ی توسعه قرار دارد.

## درباره پروژه

**Sprust یک کتابخانه‌ی انعطاف‌پذیر است، نه یک فریم‌ورک.**

هدف این پروژه این است که امکانات Bot API سروش پلاس را به شکل ساده، asynchronous و type-safe در اختیار برنامه‌نویس Rust قرار دهد؛ بدون اینکه ساختار یا معماری خاصی را به پروژه تحمیل کند.

Sprust قرار نیست تصمیم بگیرد که ربات شما چطور کار کند.
نحوه‌ی مدیریت commandها، پیام‌ها، state و منطق برنامه بر عهده‌ی خود توسعه‌دهنده است.

## وضعیت پروژه

در حال حاضر بخش‌های اصلی ارتباط با Bot API در حال پیاده‌سازی هستند.

### پیاده‌سازی شده

* [x] ساخت `Client`
* [x] مدیریت Token ربات
* [x] ارسال درخواست‌های HTTP به Bot API
* [x] تبدیل پاسخ‌های JSON به Typeهای Rust
* [x] `getMe`
* [x] `getUpdates`
* [x] Long Polling با `poll`
* [x] مدیریت `offset` در Polling
* [x] دریافت و پردازش `Update`
* [x] `sendMessage`
* [x] مدیریت خطاهای HTTP و JSON

### در حال توسعه

* [ ] پشتیبانی کامل‌تر از انواع Update
* [ ] پشتیبانی کامل‌تر از انواع Message
* [ ] امکانات بیشتر برای ارسال پیام
* [ ] Command و Message helpers
* [ ] Webhook
* [ ] File و Media API
* [ ] Callback Query
* [ ] سایر متدهای Bot API

## نصب

در حال حاضر Sprust از طریق Git قابل دریافت است:

```toml
[dependencies]
sprust = { git = "https://github.com/mrZharf/sprust" }
```

## استفاده

برای مثال، دریافت اطلاعات ربات:

```rust
use sprust::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("BOT_TOKEN")?;

    let bot = Client::new(token);

    let me = bot.get_me().await?;

    println!("{:#?}", me);

    Ok(())
}
```

### Polling

Sprust می‌تواند با استفاده از `poll` به صورت Long Polling منتظر Updateهای جدید بماند:

```rust
use sprust::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("BOT_TOKEN")?;

    let bot = Client::new(token);

    bot.poll(|update| async move {
        println!("{:#?}", update);
    })
    .await?;

    Ok(())
}
```

`poll` خودش `offset` را مدیریت می‌کند و هر `Update` را به handler تحویل می‌دهد.

### پاسخ به `/start`

Sprust منطق commandها را خودش تعیین نمی‌کند. برای مثال، می‌توانی در برنامه‌ی خودت `/start` را بررسی کنی:

```rust
if let Some(message) = update.message {
    if message.text.as_deref() == Some("/start") {
        bot.send_message(
            message.chat.id,
            "سلام 👋\nبه ربات خوش اومدی!",
        ).await?;
    }
}
```

این یعنی Sprust فقط ابزار لازم برای ارتباط با API را فراهم می‌کند و تصمیم‌گیری درباره‌ی رفتار ربات بر عهده‌ی برنامه‌ی شماست.

## معماری

ساختار کلی Sprust به شکل ساده‌ای طراحی شده است:

```text
Bot Token
    │
    ▼
  Client
    │
    ▼
HTTP Request
    │
    ▼
Soroush Plus Bot API
    │
    ▼
JSON Response
    │
    ▼
Rust Types
```

تمرکز کتابخانه روی فراهم کردن یک رابط Rustی، asynchronous و type-safe برای Bot API سروش پلاس است.

## فلسفه پروژه

Sprust قرار نیست یک سیستم بزرگ و پیچیده برای ساخت ربات باشد.

هدف این است که توسعه‌دهنده بتواند از APIهای Sprust به عنوان building block استفاده کند و خودش معماری مورد نیاز پروژه را بسازد.

بنابراین:

* Sprust فریم‌ورک نیست.
* معماری خاصی را تحمیل نمی‌کند.
* منطق commandها را خودش مدیریت نمی‌کند.
* مدیریت state را به برنامه‌ی کاربر واگذار می‌کند.
* قرار نیست همه‌چیز را پشت abstractionهای سنگین پنهان کند.

## مسیر توسعه

برنامه‌ی توسعه‌ی فعلی:

1. تکمیل مدل‌های Bot API
2. گسترش `getUpdates`
3. بهبود Polling و مدیریت Updateها
4. توسعه‌ی امکانات ارسال پیام
5. پشتیبانی از File و Media
6. Webhook
7. بهبود مدیریت خطاها
8. مستندات و مثال‌های بیشتر

## مستندات

مستندات رسمی Bot API سروش پلاس مبنای توسعه‌ی این پروژه هستند.

## مجوز

این پروژه تحت مجوز **MIT** منتشر شده است.

© 2026 mrZharf
[mrZharf] ~/sprust
