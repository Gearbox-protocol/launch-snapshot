# Launch snapshots

This repo is dedicated to presenting full transparency on how snapshots have been done leading up to DAO-First launch of Gearbox Protocol. You can find more information on the stages this is covering: https://docs.gearbox.finance/overview/launch-phases. 

### Credit account mining snapshot

* The list was snapshotted at block 13650000 (November 20).
* Dune queries are here - https://dune.xyz/queries/205069?block_cutoff=13650000, https://dune.xyz/queries/205073, https://dune.xyz/queries/199373, https://dune.xyz/queries/297547, https://dune.xyz/queries/297531.
* Jupyter notebook on Google Collab to retrieve snapshot voters list: https://colab.research.google.com/drive/1RzVVfKiwrBIGiANfH8695Lmew4ZE3U47?usp=sharing
* TXT file with the addresses from the initial snapshot: https://github.com/Gearbox-protocol/launch-snapshot/tree/master/credit_account_snapshot
* **Forum discussion on the list** - https://gov.gearbox.fi/t/credit-account-mining-snapshot-list/.

### Kovan testers snapshot

The snapshot has been taken on block #28668076. Link to snapshot is [here](https://docs.google.com/spreadsheets/d/1xQzl_pqFtP8XLZ0cx1ds-X_nWu68sthTg8ELKEf3QDk/edit#gid=689096851).

### Discord snapshot 

The snapshot has been takeon 6 December 17-00 UTC. Link to snapshot is [here](https://docs.google.com/spreadsheets/d/1xQzl_pqFtP8XLZ0cx1ds-X_nWu68sthTg8ELKEf3QDk/edit#gid=813810144).

## Gearbox tester snapshot tool

To check the correctness of kovan snapshot, you can run testet snapshot tool.

#### Requirements
1. Installed rust compiler (More: https://www.rust-lang.org/tools/install)
2. API for any ethereum provider. For, example Infura or Alchemy

#### How to install
1. Clone this repo `git clone git@github.com:Gearbox-protocol/launch-snapshot.git`
2. Go to `tester_snapshot_tool`:  `cd tester_snapshot_tool`
3. Copy `.env.sample` to `.env` with `cp .env.sample .env`
4. Fill `ETH_PROVIDER=`with your **KOVAN** API KEY
5. Run `cargo run`

After execution you can find file `testers.csv` in the same folder.

#### Disclaimer

These details can still be subject to changes, please see https://gov.gearbox.fi/ for latest.
