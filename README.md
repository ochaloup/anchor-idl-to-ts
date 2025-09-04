# anchor-idl-to-ts

A tool for generating a TS file from a standalone Anchor IDL JSON file.

**Motivation:** Sometimes after [Anchor JSON IDL](https://www.anchor-lang.com/docs/basics/idl)
                is downloaded from chain you need a TS file to build your client.

This tool tries to mimic how the Anchor CLI generates the TS file,
i.e., running `anchor build` emits the IDL JSON file (`target/idl/<name>.json>`)
and the TS file (`target/types/<name>.ts`).
When taking the IDL JSON file as input, this tool should generate the TS file
as the `anchor build` would.

The code that Anchor uses to generate the TS file is here:
https://github.com/solana-foundation/anchor/blob/18d0ca0ce9b78c03ef370406c6ba86e28e4591ab/cli/src/lib.rs#L2594

## Anchor version differences

Anchor has been changing the IDL format over time.
This tool supports IDL from before Anchor 0.30.x and after.
The one of the differences is that Anchor IDL 0.30.x+ uses `metadata` field,
requires having defined `address` field to bound the generated IDL to a specific program ID.
The version of Anchor TypeScript library [@coral-xyz/anchor](https://www.npmjs.com/package/@coral-xyz/anchor)
is bound to the particular IDL version and its format. There is no backward compatibility.

## Example to generate TS file

Let's say to use the IDL of the [voter-stake-registry](https://github.com/blockworks-foundation/voter-stake-registry).
A plugin for [SPL Governance](https://github.com/solana-labs/solana-program-library/tree/master/governance).
The VSR is build with Anchor version *0.26.x*.

When the VSR program maintainer deployed the program it uploaded the Anchor IDL to be stored on-chain
(see [Anchor CLI IDL init](https://www.anchor-lang.com/docs/references/cli#idl-init)).

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

Anchor in the newer versions (0.30.x+) changed the IDL format
and added CLI functionality to convert the IDL from older format to the newer one.

```bash
# convert command is available in Anchor 0.31.x+
avm use 0.31.0

anchor idl convert ./vsr-idl.json -o ./vsr-idl-0.31.json --program-id 4Q6WW2ouZ6V3iaNm56MTd5n2tnTm4C5fiH8miFHnAFHo
```

*NOTE:* The `--program-id` is required to add the `metadata` field to the converted IDL.
        Otherwise, you are expected to get 'Error: Program id missing in `idl.metadata.address` field'.

With new IDL file `vsr-idl-0.31.json` we can generate the TS file compatible with `@coral-xyz/anchor` version 0.31.x.

```bash
cargo run --bin anchor-idl-to-ts ./vsr-idl-0.31.json -o ./vsr-idl-0.31.ts
```