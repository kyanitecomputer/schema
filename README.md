# schema

The shared API and message schema for the [Kyanite](https://github.com/kyanitecomputer)
stack.

> **Status:** experimental — expect breaking changes.

## Overview

`schema` owns the Protocol Buffers definitions (`schema.v1`) that the Kyanite
components speak to each other, and generates language bindings for **Go**,
**TypeScript**, and **Rust** from a single source of truth. Wire data is
validated at runtime with [protovalidate](https://github.com/bufbuild/protovalidate)
against the `buf.validate` rules compiled into the descriptors.

## Layout

```
schema/
├── buf.yaml          buf module configuration
├── buf.gen.yaml      code generation (managed mode; Go/TS/Rust)
├── schema/v1/        the .proto definitions (package schema.v1)
└── gen/
    ├── go/           generated Go (protobuf + vtprotobuf + connect-go)
    ├── ts/           generated TypeScript (@kyanite/schema, bufbuild/es)
    └── rust/         generated Rust (kyanite-schema)
```

The Go bindings are importable under the vanity path
`src.kyanite.computer/schema/gen/go`.

## Regenerating

Requires [buf](https://buf.build) (and network access for remote plugins):

```sh
buf dep update
buf lint
buf generate
```

Commit the `.proto` change and the regenerated `gen/` output together.

## Contributing

See the org-wide [CONTRIBUTING guide](https://github.com/kyanitecomputer/.github/blob/main/CONTRIBUTING.md).
Contributions are dual-licensed.

## Security

See the org-wide [SECURITY policy](https://github.com/kyanitecomputer/.github/blob/main/SECURITY.md).

## License

Dual-licensed under either of Apache-2.0 ([LICENSE-APACHE](LICENSE-APACHE)) or
MIT ([LICENSE-MIT](LICENSE-MIT)) at your option.
