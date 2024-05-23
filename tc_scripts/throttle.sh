# Put your interface name here
INTERFACE=lo
LOSS_RATE=$1
DELAY=$2
BANDWIDTH=$3

# Display the configuration
echo "Configured $INTERFACE with:"
echo " - Loss rate: $LOSS_RATE%"
echo " - Delay: $DELAY ms"
echo " - Bandwidth limit: $BANDWIDTH Mbit"
sudo tc qdisc replace dev $INTERFACE root netem loss $LOSS_RATE% delay ${DELAY}ms rate ${BANDWIDTH}Mbit
