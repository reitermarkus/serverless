# Dependencies

`cross` needs to be installed from `master`, since the stable version does not yet include the Docker image we need:

```bash
git clone http://github.com/rust-embedded/cross /tmp/cross
cd /tmp/cross
cargo install --path . --force
./build-docker-image.sh armv7-unknown-linux-gnueabihf
cd -
rm -r /tmp/cross
```

# Deployment

To deploy, first set up SSH access to the Raspberry Pi, then run

```bash
./deploy <raspberry_pi_hostname_or_ip>
```

# Testing

For testing, run

```bash
./run-remote <raspberry_pi_hostname_or_ip>
```

which will display the log.
