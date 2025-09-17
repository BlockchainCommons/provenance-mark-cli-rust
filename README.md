# Blockchain Commons `provenance` Command Line Interface

<!--Guidelines: https://github.com/BlockchainCommons/secure-template/wiki -->

### _by Wolf McNally_

---

`provenance` is a command line tool for creating and managing [provenance mark](https://github.com/BlockchainCommons/Research/blob/master/papers/bcr-2025-001-provenance-mark.md) chains.

* Creates a new provenance mark chain, including the genesis mark.
* Adds a new mark to an existing chain.
* Prints marks from a chain in a form that can be published.
* DOES NOT currently support assigning or parsing the `info` field of a mark.
* DOES NOT currently support validating the integrity of a mark chain.

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

**NOTE:** Once a mark has been generated, the `generator.json` file is updated to the next sequence number and the random number generator's state is updated. The tool does not provide a way to roll back to a previous state, so if you want to experiment with generating the same mark multiple times, you should back up the `generator.json` file first, or consider using Git to manage the chain directory (in a private repo!)

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

The `generator.json` file is updated, and the new mark is written as a new file to the `marks` directory.

```bash
tree mychain

│ mychain
│ ├── generator.json
│ └── marks
│     ├── mark-0.json
│     └── mark-1.json
```

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
