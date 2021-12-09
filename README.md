# Launch snapshots

## Credit account mining snapshot

## Kovan testers snapshot

## Gearbox tester snapshot tool
To check the correctness of kovan snapshot, you can run testet snapshot tool.

### Requirements
1. Installed rust compiler (More: https://www.rust-lang.org/tools/install)
2. API for any ethereum provider. For, example Infura or Alchemy

### How to install
1. Clone this repo `git clone git@github.com:Gearbox-protocol/launch-snapshot.git`
2. Go to `tester_snapshot_tool`:  `cd tester_snapshot_tool`
3. Copy `.env.sample` to `.env` with `cp .env.sample .env`
4. Fill `ETH_PROVIDER=`with your **KOVAN** API KEY
5. Run `cargo run`

After execution you can find file `testers.csv` in the same folder.

### Disclaimer
