<img align="right" width="150px" height="150px" src="./assets/logo.png" />

# yachay

<h5 align="center">optimized and ergonomic logging</h5>

> From Quechua [yachay](https://en.wikipedia.org/wiki/Yachay) means "knowledge" or "to know", represents understanding
> and awareness.

## Features

- **High Performance** – Low allocation and fast execution [benchmark](TODO)
- **Structured Logging** – Storage-agnostic, with initial support for [TimescaleDB](https://www.timescale.com/)
- **Multi-Language Support** – Available for [C# (.NET)](https://www.nuget.org/packages/yachay-log) and
  [JavaScript](https://www.npmjs.com/package/yachay), with more to come

## Motivation

This project's premise is that traditional string-based logging often leads might lead to inefficient resource usage and
makes it difficult to categorize errors. Some applications have historically used error codes to address this issue, a
practice still employed today, as seen in
[SQL Server error codes](https://learn.microsoft.com/en-us/sql/relational-databases/errors-events/database-engine-events-and-errors-0-to-999?view=sql-server-ver16).

However, error codes can be difficult to read and maintain. `yachay` aims to provide the best of both worlds:

- **Readable Debugging** – Uses human-readable strings in `stdout` during debugging.
- **Efficient Production Logging** – Employs structured error codes in production for categorization and analysis.
- **Automated Error Dictionaries** – Generates error dictionaries at build time to automate maintenance.
- **Optimized Serialization** – Ensures high-performance logging with minimal overhead.

## License

This project contains both a **server-side service** and **client libraries**, each with different licenses:

- **The service (`/service`) is licensed under the Server Side Public License (SSPL) v1.** See
  [LICENSE.txt](./LICENSE.txt).
- **All client libraries (`/clients/*`) are licensed under the MIT License.** Each library has its own `LICENSE.txt`.

Please ensure compliance with the respective licenses before using or distributing this software.
