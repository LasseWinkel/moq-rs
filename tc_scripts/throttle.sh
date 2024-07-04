LOSS_RATE=$1
DELAY=$2
BANDWIDTH=$3
NETWORK_NAMESPACE=$4

if [ "$NETWORK_NAMESPACE" = "ns-rs" ]; then
    INTERFACE="app-rs"
elif [ "$NETWORK_NAMESPACE" = "ns-js" ]; then
    INTERFACE="app-js"
elif [ "$NETWORK_NAMESPACE" = "ns-js-sub" ]; then
	NETWORK_NAMESPACE="ns-rs"
    INTERFACE="app-rs-sub"
fi

# Display the configuration
echo "Configured $INTERFACE at $NETWORK_NAMESPACE with:"
echo " - Loss rate: $LOSS_RATE%"
echo " - Delay: $DELAY ms"
echo " - Bandwidth limit: $BANDWIDTH Mbit"
sudo ip netns exec $NETWORK_NAMESPACE tc qdisc replace dev $INTERFACE root netem loss $LOSS_RATE% delay ${DELAY}ms rate ${BANDWIDTH}Mbit
