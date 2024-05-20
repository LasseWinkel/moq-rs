# Put your interface name here
INTERFACE=lo
LOSS_RATE=$1

echo "tc qdisc replace dev $INTERFACE root netem loss $LOSS_RATE%"
sudo tc qdisc replace dev $INTERFACE root netem loss $LOSS_RATE%
