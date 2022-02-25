
Refresh the testnet 
```bash
ssh 159.65.203.73
cd liberland_node
git pull origin dev
cargo build --release
sudo systemd restart testnet-liberland
```


