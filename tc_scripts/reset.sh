# Path to the JSON file
config_file="config.json"

# Read values from JSON file using jq
NETWORK_INTERFACE_PUBLISHER=$(jq -r '.networkInterfacePublisher' "$config_file")
NETWORK_INTERFACE_SERVER=$(jq -r '.networkInterfaceServer' "$config_file")
NETWORK_INTERFACE_REMOTE_SUBSCRIBER=$(jq -r '.networkInterfaceRemoteSubscriber' "$config_file")
ARE_LINUX_NETWORK_NAMESPACES_USED=$(jq -r '.areLinuxNetworkNamespacesUsed' "$config_file")


NETWORK_NAMESPACE=$1

if [ "$NETWORK_NAMESPACE" = "ns-rs" ]; then
    INTERFACE=$NETWORK_INTERFACE_SERVER
elif [ "$NETWORK_NAMESPACE" = "ns-js" ]; then
    INTERFACE=$NETWORK_INTERFACE_PUBLISHER
elif [ "$NETWORK_NAMESPACE" = "ns-js-sub" ]; then
	NETWORK_NAMESPACE="ns-rs"
    INTERFACE=$NETWORK_INTERFACE_REMOTE_SUBSCRIBER
fi

if [ $ARE_LINUX_NETWORK_NAMESPACES_USED = true ]; then
	if sudo ip netns exec $NETWORK_NAMESPACE tc qdisc show dev $INTERFACE | grep netem; then
		echo "ip netns exec $NETWORK_NAMESPACE tc qdisc del dev $INTERFACE root"
		sudo ip netns exec $NETWORK_NAMESPACE tc qdisc del dev $INTERFACE root
	else
		echo "no netem rule"
	fi
else
	if sudo tc qdisc show dev $INTERFACE | grep netem; then
		echo "tc qdisc del dev $INTERFACE root"
		sudo tc qdisc del dev $INTERFACE root
	else
		echo "no netem rule"
	fi
fi



