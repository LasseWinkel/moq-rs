# Put your interface name here
INTERFACE=lo

if tc qdisc show dev $INTERFACE | grep netem; then
    echo "tc qdisc del dev $INTERFACE root"
    sudo tc qdisc del dev $INTERFACE root
else
    echo "no netem rule"
fi
