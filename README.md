# anchor-idl-to-ts

A tool for generating a TypeScript (TS) file from a standalone Anchor IDL JSON file.

**Motivation:** Sometimes, after downloading an [Anchor JSON IDL](https://www.anchor-lang.com/docs/basics/idl)
                from the chain, you need a corresponding TS file to build your client.

This tool mimics how the Anchor CLI generates a TS file. Normally, running `anchor build` produces both
the IDL JSON file (`target/idl/<name>.json`) and the TS file (`target/types/<name>.ts`).
Given only the IDL JSON file as input, this tool generates the TS file exactly as `anchor build` would.

The Anchor code used to generate the TS file can be found here:
[https://github.com/solana-foundation/anchor/blob/18d0ca0ce9b78c03ef370406c6ba86e28e4591ab/cli/src/lib.rs#L2594](https://github.com/solana-foundation/anchor/blob/18d0ca0ce9b78c03ef370406c6ba86e28e4591ab/cli/src/lib.rs#L2594)

---

## Anchor version differences

The Anchor IDL format has changed over time.
This tool supports IDLs both from before Anchor 0.30.x and from later versions.

One key difference in Anchor 0.30.x+ is the use of the `metadata` field, which requires an `address` field to bind the generated IDL to a specific program ID.

The [@coral-xyz/anchor](https://www.npmjs.com/package/@coral-xyz/anchor) TypeScript library version is tied to the corresponding Anchor IDL version and format. There is no backward compatibility.

---

## Example: Generating a TS file

As an example, consider the [voter-stake-registry (VSR)](https://github.com/blockworks-foundation/voter-stake-registry), a plugin for [SPL Governance](https://github.com/solana-labs/solana-program-library/tree/master/governance).
The VSR program is built with Anchor version *0.26.x*.

When the VSR program maintainer deployed the program, the Anchor IDL was uploaded and stored on-chain (see [Anchor CLI `idl init`](https://www.anchor-lang.com/docs/references/cli#idl-init)).

*NOTE:* By default, Anchor includes the IDL instruction in the program binary.
        However, you can disable it by enabling the
        [`no-idl` cargo feature](https://github.com/solana-foundation/anchor/blob/5300d7cf8aaf52da08ce331db3fc8182cd821228/lang/src/idl.rs#L17-L18).

```
voter-stake-registry:  4Q6WW2ouZ6V3iaNm56MTd5n2tnTm4C5fiH8miFHnAFHo
```

```bash
anchor --provider.cluster mainnet idl fetch 4Q6WW2ouZ6V3iaNm56MTd5n2tnTm4C5fiH8miFHnAFHo > ./vsr-idl.json
```
From the downloaded IDL file `vsr-idl.json` we can generate the TS file compatible with `@coral-xyz/anchor` version 0.26.x.

```bash
cargo run --bin anchor-idl-to-ts ./vsr-idl.json
```

### Example to convert for newer Anchor versions

In newer versions of Anchor (0.30.x+), the IDL format has changed, and the CLI now provides
functionality to convert an older IDL format into the newer one.

```bash
# 'convert' cli argument is available only in Anchor 0.31.x+
avm use 0.31.0

anchor idl convert ./vsr-idl.json -o ./vsr-idl-0.31.json --program-id 4Q6WW2ouZ6V3iaNm56MTd5n2tnTm4C5fiH8miFHnAFHo
```


*NOTE:* The `--program-id` flag is required to populate the `metadata` field in the converted IDL.
        Without it, you will encounter the error: "Error: Program id missing in \`idl.metadata.address\` field".

With the new IDL file `vsr-idl-0.31.json`, we can generate a TS file compatible with `@coral-xyz/anchor` version 0.31.x.

```bash
cargo run --bin anchor-idl-to-ts ./vsr-idl-0.31.json -o ./vsr-idl-0.31.ts
```


-----
Forked from [nicholas-ewasiuk/simple-anchor-idl-ts](https://github.com/nicholas-ewasiuk/simple-anchor-idl-ts)
Credits to [`anchor-idl`](https://github.com/saber-hq/anchor-gen) that does all the heavy lifting.
