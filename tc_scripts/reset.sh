NETWORK_NAMESPACE=$1

if [ "$NETWORK_NAMESPACE" = "ns-rs" ]; then
    INTERFACE="app-rs"
elif [ "$NETWORK_NAMESPACE" = "ns-js" ]; then
    INTERFACE="app-js"
fi

if sudo ip netns exec $NETWORK_NAMESPACE tc qdisc show dev $INTERFACE | grep netem; then
    echo "ip netns exec $NETWORK_NAMESPACE tc qdisc del dev $INTERFACE root"
    sudo ip netns exec $NETWORK_NAMESPACE tc qdisc del dev $INTERFACE root
else
    echo "no netem rule"
fi
