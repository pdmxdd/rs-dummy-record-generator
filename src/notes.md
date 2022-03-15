## generate random number less than 10,000,000:

date -I'ns' | sed --expression='s/.*,//' | sed --expression='s/-.*$//'

## downloads the list of awful passwords & then selects a random one

```bash
wget https://raw.githubusercontent.com/danielmiessler/SecLists/master/Passwords/Common-Credentials/10-million-password-list-top-1000000.txt

cat 10-million-password-list-top-1000000.txt | sed -n -e $(date -I"ns" | sed --e="s/.*,[0-9]\{3\}//" | sed --e="s/-.*$//")'{p;q}'
```

```bash
cat 10-million-password-list-top-1000000.txt | sed --e 's/$/,/' > bad-passwords.csv
```