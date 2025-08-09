download-maelstrom:
    curl -sL https://github.com/jepsen-io/maelstrom/releases/download/v0.2.4/maelstrom.tar.bz2 | tar -xz
    mv maelstrom maelstrom-tmp
    mkdir -p lib
    mv ./maelstrom-tmp/lib/maelstrom.jar lib/
    mv ./maelstrom-tmp/maelstrom .
    chmod +x maelstrom
    rm -rf maelstrom-tmp


echo:
    cargo b --bin echo && ./maelstrom test -w echo --bin target/debug/echo --node-count 1 --time-limit 10

generate:
    cargo b --bin generate && ./maelstrom test -w unique-ids --bin target/debug/generate --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition

broadcast-single:
    cargo b --bin broadcast && ./maelstrom test -w broadcast --bin target/debug/broadcast --node-count 1 --time-limit 20 --rate 10

broadcast-multi:
    cargo b --bin broadcast && ./maelstrom test -w broadcast --bin target/debug/broadcast --node-count 5 --time-limit 20 --rate 10
