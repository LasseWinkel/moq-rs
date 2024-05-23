# Put your interface name here
INTERFACE=lo
LOSS_RATE=$1
DELAY=$2

echo "tc qdisc replace dev $INTERFACE root netem loss $LOSS_RATE% delay ${DELAY}ms"
sudo tc qdisc replace dev $INTERFACE root netem loss $LOSS_RATE% delay ${DELAY}ms
