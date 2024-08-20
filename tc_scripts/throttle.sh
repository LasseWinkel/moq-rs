# Path to the JSON file
config_file="../config.json"

# Read values from JSON file using jq
NETWORK_INTERFACE_PUBLISHER=$(jq -r '.networkInterfacePublisher' "$config_file")
NETWORK_INTERFACE_SERVER=$(jq -r '.networkInterfaceServer' "$config_file")
NETWORK_INTERFACE_REMOTE_SUBSCRIBER=$(jq -r '.networkInterfaceRemoteSubscriber' "$config_file")
ARE_LINUX_NETWORK_NAMESPACES_USED=$(jq -r '.areLinuxNetworkNamespacesUsed' "$config_file")

LOSS_RATE=$1
DELAY=$2
BANDWIDTH=$3
NETWORK_NAMESPACE=$4

if [ "$NETWORK_NAMESPACE" = "ns-rs" ]; then
    INTERFACE=$NETWORK_INTERFACE_SERVER
elif [ "$NETWORK_NAMESPACE" = "ns-js" ]; then
    INTERFACE=$NETWORK_INTERFACE_PUBLISHER
elif [ "$NETWORK_NAMESPACE" = "ns-js-sub" ]; then
	NETWORK_NAMESPACE="ns-rs"
    INTERFACE=$NETWORK_INTERFACE_REMOTE_SUBSCRIBER
fi

if [ $ARE_LINUX_NETWORK_NAMESPACES_USED = true ]; then
	sudo ip netns exec $NETWORK_NAMESPACE tc qdisc replace dev $INTERFACE root netem loss $LOSS_RATE% delay ${DELAY}ms rate ${BANDWIDTH}Mbit
	echo "Configured $INTERFACE at $NETWORK_NAMESPACE with:"
	echo " - Loss rate: $LOSS_RATE%"
	echo " - Delay: $DELAY ms"
	echo " - Bandwidth limit: $BANDWIDTH Mbit"
else
	sudo tc qdisc replace dev $INTERFACE root netem loss $LOSS_RATE% delay ${DELAY}ms rate ${BANDWIDTH}Mbit
	echo "Configured $INTERFACE with:"
	echo " - Loss rate: $LOSS_RATE%"
	echo " - Delay: $DELAY ms"
	echo " - Bandwidth limit: $BANDWIDTH Mbit"
fi
