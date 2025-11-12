# Blockchain Commons `provenance` Command Line Interface

<!--Guidelines: https://github.com/BlockchainCommons/secure-template/wiki -->

### _by Wolf McNally_

---

`provenance` is a command line tool for creating and managing [provenance mark](https://github.com/BlockchainCommons/Research/blob/master/papers/bcr-2025-001-provenance-mark.md) chains.

* Creates a new provenance mark chain, including the genesis mark.
* Adds a new mark to an existing chain.
* Prints marks from a chain in a form that can be published.
* Validates one or more provenance marks, checking for integrity issues.
* DOES NOT currently support assigning or parsing the `info` field of a mark.

**NOTE:** This tool is currently in a pre-release state, and is under active development. Use at your own risk.

**NOTE:** The format of provenance marks as described in the White Paper is believed to be stable, but the format of allowable data in the `info` field is still under discussion and may change. It is safe to use a CBOR `text` string in the `info` field as a human-readable hashed-in comment (once this tool supports adding it), but other data types are not yet specified.

## Related Projects

* [Provenance Mark White Paper](https://github.com/BlockchainCommons/Research/blob/master/papers/bcr-2025-001-provenance-mark.md)
* [Provenance Mark Rust Crate](https://github.com/BlockchainCommons/provenance-mark-rust)
* [Provenance Mark Swift Package](https://github.com/BlockchainCommons/Provenance)

## Installation

To install from crates.io, run:

```bash
cargo install provenance-mark-cli
```

To install from source, clone this repo, change to its root directory and run:

```bash
cargo install --path .
```

## Documentation

A summary of options for `provenance` and its sub-commands can be found by running `provenance --help` or `provenance <subcommmand> --help`.

## Starting a New Provence Mark Chain

The `provenance new` command is used to create a new directory in which the seed and state of the provenance mark chain will be stored, along with the marks themselves.

- The absolute or relative path of a new directory must be provided as an argument, which the tool will create. If the specified directory already exists, an error will be returned.
- The `--seed` option can be used to provide a seed for the mark chain, encoded as base64. If not supplied, a random seed is generated.
- The `--resolution` option can be used to specify the resolution of the provenance mark chain (`low`, `medium`, default: `quartile`, or `high`)
- The `--comment` option can be used to provide a comment for the genesis mark. (default: `Genesis mark.`)

**A Note on Comments:** Comments are not part of the mark itself (i.e., in its `info` field), but are included in the provenance mark chain for informational purposes, and can be edited without invalidating the chain.

**A Note on the Info Field:** The current state of the `provenance` tool does not afford the ability to assign the `info` field of a new mark, nor read or parse it in any way. This will be added in a future release.

Running `provenance new` will create a new directory with the specified name, and write the genesis mark to a file named `mark-0.json` in the `marks` subdirectory of the new directory:

```bash
provenance new mychain
```

The output will look something like this:

```
│ Provenance mark chain created at: /Users/wolf/mychain
│
│ Mark 0 written to: /Users/wolf/mychain/marks/mark-0.json
│
│ ---
│
│ 2025-01-27T21:59:52Z
│
│ #### ur:provenance/lfaohdftldguvoglatjpmdhnbkzctthlkobyguehwlsefpamsnnntidsfzbglefmhdnblnpyasjltynldtfwwnaapmadzcsrctlsbdpsztonstolgllnhnpavsglclgamero
│
│ #### `🅟 PLAY WASP FLUX SWAN`
│
│ 🅟 💎 🦄 🍓 🧢
│
│ Genesis mark.
```

The format is:

```
│ The full path of the new directory.
│
│ Where the genesis mark (mark-0.json) was written.
│
│ ---
│
│ Date - ISO-8601 format.
│
│ Provenance Mark UR - This is the complete data structure in UR format.
│ It is marked with a `####` for Markdown systems like GitHub which
│ automatically add anchors to headers.
│
│ Provenance Mark Bytewords Identifier - This is the bytewords identifier
│ for the mark. It is marked with a `####` for Markdown systems like
│ GitHub which automatically add anchors to headers.
│
│ Provenance Mark Bytemoji Identifier - This is the bytemoji identifier
│ for the mark. Anchors usually cannot be created from emoji,
│ so there is no `####` here.
│
│ Comment
```

Everything from the `---` down can be copied and pasted into a Markdown file, a text file, or any other document, and published as, for example, a GitHub Gist.

## Directory Structure

The created directory will look like this:

```bash
tree mychain

│ mychain
│ ├── generator.json
│ └── marks
│     └── mark-0.json
```

`generator.json` contains the state of the mark chain, including the seed, chain ID, and next sequence number.

**NOTE:** It is vitally important that the `generator.json` file is kept secret. It should not be shared or published. It is used to generate the next mark in the chain. If it is lost or corrupted, the chain cannot be continued. If it is stolen, an attacker could forge marks in the chain.

```bash
cat mychain/generator.json

│ {
│   "res": 2,
│   "seed": "+3viDXTkbHL99p2LYQhiyqtFqr4v4mYpDvXtAmqhzME=",
│   "chainID": "iVPiTgdylWAK/dFddhFTMQ==",
│   "nextSeq": 1,
│   "rngState": "RKs3oewHj+NH5HnNCXJW9z0GCLSvoNx+EwfHKG344NM="
│ }
```

`marks` is a directory containing the mark files, named `mark-0.json` (the genesis mark), `mark-1.json`, etc. There is nothing secret in these files, but they contain redundant information and are not intended to be human-readable. (See the `print` sub-command below for a human-readable version of a mark.)

The only field of the mark that you may edit is the `comment` field. This is not part of the mark itself, but is included in the provenance mark chain for informational purposes.

```sh
cat mychain/marks/mark-0.json

│ {
│   "ur": "ur:provenance/lfaohdftldguvoglatjpmdhnbkzctthlkobyguehwlsefpamsnnntidsfzbglefmhdnblnpyasjltynldtfwwnaapmadzcsrctlsbdpsztonstolgllnhnpavsglclgamero",
│   "bytewords": "🅟 PLAY WASP FLUX SWAN",
│   "bytemoji": "🅟 💎 🦄 🍓 🧢",
│   "comment": "Genesis mark.",
│   "mark": {
│     "seq": 0,
│     "date": "2025-01-27T21:59:52Z",
│     "res": 2,
│     "chain_id": "iVPiTgdylWAK/dFddhFTMQ==",
│     "key": "iVPiTgdylWAK/dFddhFTMQ==",
│     "hash": "q+xDzagOYatKFOk+Yt0aHw=="
│   }
│ }
```

## Adding a New Mark to a Chain

The `provenance next` command is used to generate the next mark in a chain.

- The path to the chain's directory as an argument is required.
- The `--comment` option can be used to provide a comment for the new mark. (default: `Blank.`)
- The `--format` option controls output format: `markdown` (default), `ur`, or `json`.
- The `--quiet` option suppresses status messages, showing only the mark data.

**NOTE:** Once a mark has been generated, the `generator.json` file is updated to the next sequence number and the random number generator's state is updated. The tool does not provide a way to roll back to a previous state, so if you want to experiment with generating the same mark multiple times, you should back up the `generator.json` file first, or consider using Git to manage the chain directory (in a private repo!)

### Default Output (Markdown)

```bash
provenance next mychain --comment "My cool new work I want to be tied to the chain."

│ Mark 1 written to: mychain/marks/mark-1.json
│
│ ---
│
│ 2025-01-27T22:19:15Z
│
│ #### ur:provenance/lfaohdftftgydnnssacmvwhprtplplzsgwcspaaygmveeoeskgdipmwfynnncswsnngoyanygmbkftdiwngoztahcltlctgeaxeoswlagroxhfwpnbmsmehybsvllgjpnett
│
│ #### `🅟 COLA TUNA CUSP WAND`
│
│ 🅟 🤑 🐶 👺 🦉
│
│ My cool new work I want to be tied to the chain.
```

### UR Output

The `--format ur` option outputs only the provenance mark UR, suitable for piping to other tools or embedding in scripts:

```bash
provenance next mychain --comment "Automated build" --format ur

│ Mark 2 written to: mychain/marks/mark-2.json
│ ur:provenance/lfaohdftfeenaadsrhghbdmukpzevorevdndfnecpdkschgsmdjknsyabzuetojnbnckprryrhpstpbkkehdmslkplhfptrhgmhsndtpjsgrwmsglnladlndvlfemdfhsstp
```

### JSON Output

The `--format json` option provides structured data for programmatic access:

```bash
provenance next mychain --comment "Release v1.0" --format json

│ Mark 3 written to: mychain/marks/mark-3.json
│ {
│   "ur": "ur:provenance/lfaohdftiegtjeiehkndyldrbzrhrsfdaoessfmuaaweeomuadnehfrtgelahhseiaaycmdarszopdzedelutebklasardfddibgjklngeistbwecaimpsuokshshdrfzssb",
│   "bytewords": "🅟 FLAP GRAY FACT LOUD",
│   "bytemoji": "🅟 🍉 🥐 👀 🚫",
│   "comment": "Release v1.0",
│   "mark": {
│     "seq": 3,
│     "date": "2025-11-12T07:49:18Z",
│     "res": 2,
│     "chain_id": "JksfJNSDpXkgZXtVZSTWcA==",
│     "key": "ZE1rZFmb9yoVub9IAjnMkw==",
│     "hash": "QVE6iQX9+1HI9dk85RDVKQ=="
│   }
│ }
```

### Quiet Mode

The `--quiet` option suppresses the status message, outputting only the mark data. This is especially useful in scripts:

```bash
provenance next mychain --comment "CI build" --format ur --quiet

│ ur:provenance/lfaohdfthgidwttblefyidgeltprdebtwfoefybwsomuynrfuehkjploykiepawfihtnenesjlfrrdiymkzmutsbfzuyosfslftnwyftmdwphddiwmcaatluhnmefsdwvwfg
```

### Updated Directory Structure

The `generator.json` file is updated, and the new mark is written as a new file to the `marks` directory.

```bash
tree mychain

│ mychain
│ ├── generator.json
│ └── marks
│     ├── mark-0.json
│     └── mark-1.json
```

## Validating Marks

The `provenance validate` command validates one or more provenance marks for integrity and chain continuity. It accepts provenance mark URs as arguments or can validate an entire chain directory.

The validator checks for:
- **Duplicates**: Exact duplicate marks in the input
- **Chain continuity**: Proper sequence numbering and hash chain integrity
- **Genesis marks**: Presence of a valid genesis mark (sequence 0)
- **Multiple chains**: Whether marks belong to different chains
- **Sequence gaps**: Missing marks in the chain

### Validating from URs

To validate marks, provide their URs as arguments:

```bash
provenance validate ur:provenance/lfaohdftdsgrctdktylsonkkcxihkggoihdktbjovwkeztpdstfggdctqddlwnecaadyktfefnfxkgcyhlqddsvdnsstdejsbzaabzgegmjejoferpiavarovyfsinfgkpny ur:provenance/lfaohdftchcegumkhlsecwurldqdsomdcmhhwzeykiimplprinaxamvlsghlwlgdihdinbrkdabamnztbbgdvakpsbaapldykovddtmhftfshtdmvyesonsstkltldtliefl ur:provenance/lfaohdftmhktonjtknetistaatdtwlpkhhceuygelrrekooldnmntatbwtyllusgeyswzowztklbtkztskmugutpjpntwdwefzhtcapekpgtwslgtpaouegebartdilnkiam
```

### Validating from a Chain Directory

The `--dir` option validates all marks in a chain directory:

```bash
provenance validate --dir mychain
```

This is convenient for validating an entire chain without manually extracting URs from each mark file.

### Exit Codes and Behavior

By default, the `validate` command:
- **Exits with code 0** (success) if marks form a single, perfect chain with no issues
- **Exits with code 1** (failure) if any issues are detected
- **Produces no output** for perfect chains (following the Unix philosophy of silence on success)

Example of a successful validation (no output):

```bash
provenance validate --dir mychain
```

(No output; exit code 0)

### Detecting Issues

The validator reports "interesting" information when issues are detected. A single contiguous chain of well-formed marks with no issues produces no output, while any exceptional states (duplicates, gaps, multiple chains, missing genesis) are reported.

#### Example: Gap in Chain

Validating marks with a missing sequence number:

```bash
provenance validate ur:provenance/lfaohdftdsgrctdktylsonkkcxihkggoihdktbjovwkeztpdstfggdctqddlwnecaadyktfefnfxkgcyhlqddsvdnsstdejsbzaabzgegmjejoferpiavarovyfsinfgkpny ur:provenance/lfaohdftmhktonjtknetistaatdtwlpkhhceuygelrrekooldnmntatbwtyllusgeyswzowztklbtkztskmugutpjpntwdwefzhtcapekpgtwslgtpaouegebartdilnkiam

│ Error: Validation failed with issues:
│ Total marks: 2
│ Chains: 1
│
│ Chain 1: 264b1f24
│   0: 13cfd320 (genesis mark)
│   2: f2ab17fc (gap: 1 missing)
```

Each mark is displayed with its sequence number and short identifier (first 4 bytes of hash). Issues are shown inline as annotations:
- `(genesis mark)` - Marks the genesis (sequence 0) mark
- `(gap: N missing)` - Indicates a sequence gap (N marks are missing before this one)
- `(date X < Y)` - Date ordering violation between marks
- Other validation issues appear similarly

### Warning Mode

The `--warn` flag allows validation to succeed even when issues are detected:

```bash
provenance validate --warn ur:provenance/lfaohdftdsgrctdktylsonkkcxihkggoihdktbjovwkeztpdstfggdctqddlwnecaadyktfefnfxkgcyhlqddsvdnsstdejsbzaabzgegmjejoferpiavarovyfsinfgkpny ur:provenance/lfaohdftmhktonjtknetistaatdtwlpkhhceuygelrrekooldnmntatbwtyllusgeyswzowztklbtkztskmugutpjpntwdwefzhtcapekpgtwslgtpaouegebartdilnkiam

│ Total marks: 2
│ Chains: 1
│
│ Chain 1: 264b1f24
│   0: 13cfd320 (genesis mark)
│   2: f2ab17fc (gap: 1 missing)
```

With `--warn`:
- Issues are reported to stdout (without "Error:" prefix)
- Command exits with code 0 (success)
- Useful for auditing without causing build failures in CI/CD pipelines

### Use Cases

- **Publishing verification**: Validate that you have a complete chain before publishing marks
- **Chain auditing**: Detect gaps or issues in archived mark collections
- **CI/CD integration**: Use in build scripts to verify provenance marks (use `--warn` to avoid build failures)
- **Cross-validation**: Verify marks received from multiple sources form a valid chain

### Validating Envelope-Wrapped Provenance Marks

The `validate` command can accept **any UR type** whose CBOR payload contains a [Gordian Envelope](https://github.com/BlockchainCommons/BCSwiftEnvelope) with a `'provenance'` assertion. This includes:

- `ur:provenance` - Direct provenance marks
- `ur:envelope` - Generic envelopes with provenance assertions
- `ur:xid` - XID Documents with provenance marks
- Any other UR type that encodes an envelope containing a provenance mark

This makes it easy to validate provenance marks that are embedded in other data structures without extracting them first.

#### Example: Validating XID Documents

[XID Documents](https://github.com/BlockchainCommons/Research/blob/master/papers/bcr-2024-010-xid.md) can include provenance marks to track their revision history. You can create and validate a chain of XID documents using the `envelope` tool:

```bash
# Generate a private key
PRVKEYS=$(envelope generate prvkeys)

# Create XID with genesis provenance mark
XID0=$(envelope xid new $PRVKEYS --generator include --date 2025-01-15)

# Advance to create mark 1
XID1=$(envelope xid provenance next --date 2025-01-20 $XID0)

# Advance to create mark 2
XID2=$(envelope xid provenance next --date 2025-01-25 $XID1)

# Validate the complete chain
provenance validate $XID0 $XID1 $XID2

│ (No output; exit code 0)
```

The validator extracts the provenance marks from each XID document's `'provenance'` assertion and validates them as a chain.

#### Detecting Issues in Envelope-Wrapped Marks

The validator detects issues in envelope-wrapped marks the same way as direct marks:

```bash
# Validate with a gap (skipping XID1)
provenance validate --warn $XID0 $XID2

│ Total marks: 2
│ Chains: 1
│
│ Chain 1: 42740037
│   0: fe88b0d1 (genesis mark)
│   2: 03668f98 (gap: 1 missing)
```

This flexibility allows provenance marks to be embedded in various document types while maintaining validation capability. See the [`envelope xid` documentation](https://github.com/BlockchainCommons/bc-envelope-cli/blob/master/docs/XID.md#working-with-provenance-marks) for more details on working with provenance marks in XID documents.

## Printing Marks

The `provenance print` command is used to print one or more marks from a chain. It requires the path to the chain's directory as an argument.

- The `--start` option can be used to specify the sequence number of the first mark to print. If not supplied, the first mark (mark 0, the genesis mark) is used.
- The `--end` option can be used to specify the sequence number of the last mark to print. If not supplied, the last mark in the chain is used.
- With no `--start` or `--end` options, all marks in the chain are printed.

```bash
provenance print mychain

│ ---
│
│ 2025-01-27T21:59:52Z
│
│ #### ur:provenance/lfaohdftldguvoglatjpmdhnbkzctthlkobyguehwlsefpamsnnntidsfzbglefmhdnblnpyasjltynldtfwwnaapmadzcsrctlsbdpsztonstolgllnhnpavsglclgamero
│
│ #### `🅟 PLAY WASP FLUX SWAN`
│
│ 🅟 💎 🦄 🍓 🧢
│
│ Genesis mark.
│
│ ---
│
│ 2025-01-27T22:19:15Z
│
│ #### ur:provenance/lfaohdftftgydnnssacmvwhprtplplzsgwcspaaygmveeoeskgdipmwfynnncswsnngoyanygmbkftdiwngoztahcltlctgeaxeoswlagroxhfwpnbmsmehybsvllgjpnett
│
│ #### `🅟 COLA TUNA CUSP WAND`
│
│ 🅟 🤑 🐶 👺 🦉
│
│ My cool new work I want to be tied to the chain.
```

## Status - Alpha

`provenance`  is currently under active development and in the alpha testing phase. It should not be used for production tasks until it has had further testing and auditing. See [Blockchain Commons' Development Phases](https://github.com/BlockchainCommons/Community/blob/master/release-path.md).

## Version History

### 0.6.0, November 12, 2025

- Add validate subcommand for validating provenance mark chains.
- Add --quiet and --format options to next subcommand.
- Update documentation.
- Align to dependencies.

### 0.5.0, September 29, 2025

- More flexible seed format handling
- Added `--quiet` option to `new` command
- Refined `--info` option handling

### 0.4.0, September 16, 2025

- Align to dependencies.
- Update documentation command line examples.

### 0.3.0, July 3, 2025

- Align to dependencies.
- Code cleanup and formatting improvements.

## Financial Support

`provenance` is a project of [Blockchain Commons](https://www.blockchaincommons.com/). We are proudly a "not-for-profit" social benefit corporation committed to open source & open development. Our work is funded entirely by donations and collaborative partnerships with people like you. Every contribution will be spent on building open tools, technologies, and techniques that sustain and advance blockchain and internet security infrastructure and promote an open web.

To financially support further development of `provenance` and other projects, please consider becoming a Patron of Blockchain Commons through ongoing monthly patronage as a [GitHub Sponsor](https://github.com/sponsors/BlockchainCommons). You can also support Blockchain Commons with bitcoins at our [BTCPay Server](https://btcpay.blockchaincommons.com/).

## Contributing

We encourage public contributions through issues and pull requests! Please review [CONTRIBUTING.md](./CONTRIBUTING.md) for details on our development process. All contributions to this repository require a GPG signed [Contributor License Agreement](./CLA.md).

### Discussions

The best place to talk about Blockchain Commons and its projects is in our GitHub Discussions areas.

[**Gordian Developer Community**](https://github.com/BlockchainCommons/Gordian-Developer-Community/discussions). For standards and open-source developers who want to talk about interoperable wallet specifications, please use the Discussions area of the [Gordian Developer Community repo](https://github.com/BlockchainCommons/Gordian-Developer-Community/discussions). This is where you talk about Gordian specifications such as [Gordian Envelope](https://github.com/BlockchainCommons/Gordian/tree/master/Envelope#articles), [bc-shamir](https://github.com/BlockchainCommons/bc-shamir), [Sharded Secret Key Reconstruction](https://github.com/BlockchainCommons/bc-sskr), and [bc-ur](https://github.com/BlockchainCommons/bc-ur) as well as the larger [Gordian Architecture](https://github.com/BlockchainCommons/Gordian/blob/master/Docs/Overview-Architecture.md), its [Principles](https://github.com/BlockchainCommons/Gordian#gordian-principles) of independence, privacy, resilience, and openness, and its macro-architectural ideas such as functional partition (including airgapping, the original name of this community).

[**Gordian User Community**](https://github.com/BlockchainCommons/Gordian/discussions). For users of the Gordian reference apps, including [Gordian Coordinator](https://github.com/BlockchainCommons/iOS-GordianCoordinator), [Gordian Seed Tool](https://github.com/BlockchainCommons/GordianSeedTool-iOS), [Gordian Server](https://github.com/BlockchainCommons/GordianServer-macOS), [Gordian Wallet](https://github.com/BlockchainCommons/GordianWallet-iOS), and [SpotBit](https://github.com/BlockchainCommons/spotbit) as well as our whole series of [CLI apps](https://github.com/BlockchainCommons/Gordian/blob/master/Docs/Overview-Apps.md#cli-apps). This is a place to talk about bug reports and feature requests as well as to explore how our reference apps embody the [Gordian Principles](https://github.com/BlockchainCommons/Gordian#gordian-principles).

[**Blockchain Commons Discussions**](https://github.com/BlockchainCommons/Community/discussions). For developers, interns, and patrons of Blockchain Commons, please use the discussions area of the [Community repo](https://github.com/BlockchainCommons/Community) to talk about general Blockchain Commons issues, the intern program, or topics other than those covered by the [Gordian Developer Community](https://github.com/BlockchainCommons/Gordian-Developer-Community/discussions) or the
[Gordian User Community](https://github.com/BlockchainCommons/Gordian/discussions).

### Other Questions & Problems

As an open-source, open-development community, Blockchain Commons does not have the resources to provide direct support of our projects. Please consider the discussions area as a locale where you might get answers to questions. Alternatively, please use this repository's [issues](./issues) feature. Unfortunately, we can not make any promises on response time.

If your company requires support to use our projects, please feel free to contact us directly about options. We may be able to offer you a contract for support from one of our contributors, or we might be able to point you to another entity who can offer the contractual support that you need.

### Credits

The following people directly contributed to this repository. You can add your name here by getting involved. The first step is learning how to contribute from our [CONTRIBUTING.md](./CONTRIBUTING.md) documentation.

| Name              | Role                     | Github                                           | Email                                 | GPG Fingerprint                                    |
| ----------------- | ------------------------ | ------------------------------------------------ | ------------------------------------- | -------------------------------------------------- |
| Christopher Allen | Principal Architect      | [@ChristopherA](https://github.com/ChristopherA) | \<ChristopherA@LifeWithAlacrity.com\> | FDFE 14A5 4ECB 30FC 5D22  74EF F8D3 6C91 3574 05ED |
| Wolf McNally      | Lead Researcher/Engineer | [@WolfMcNally](https://github.com/wolfmcnally)   | \<Wolf@WolfMcNally.com\>              | 9436 52EE 3844 1760 C3DC  3536 4B6C 2FCF 8947 80AE |

## Responsible Disclosure

We want to keep all of our software safe for everyone. If you have discovered a security vulnerability, we appreciate your help in disclosing it to us in a responsible manner. We are unfortunately not able to offer bug bounties at this time.

We do ask that you offer us good faith and use best efforts not to leak information or harm any user, their data, or our developer community. Please give us a reasonable amount of time to fix the issue before you publish it. Do not defraud our users or us in the process of discovery. We promise not to bring legal action against researchers who point out a problem provided they do their best to follow the these guidelines.

### Reporting a Vulnerability

Please report suspected security vulnerabilities in private via email to ChristopherA@BlockchainCommons.com (do not use this email for support). Please do NOT create publicly viewable issues for suspected security vulnerabilities.

The following keys may be used to communicate sensitive information to developers:

| Name              | Fingerprint                                        |
| ----------------- | -------------------------------------------------- |
| Christopher Allen | FDFE 14A5 4ECB 30FC 5D22  74EF F8D3 6C91 3574 05ED |

You can import a key by running the following command with that individual’s fingerprint: `gpg --recv-keys "<fingerprint>"` Ensure that you put quotes around fingerprints that contain spaces.
