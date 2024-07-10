#! /bin/bash

function echo_help {
  echo """
Usage:

bash run-ephemeral-it.sh <pod_name>
"""
}

POD_NAME=$1
#POD_IP=$(kubectl get pod $POD_NAME --template '{{.status.podIP}}')

if [[ $POD_NAME == "" ]]; then
  echo "Pod name is missing"
  echo_help
  exit 1
fi

#iptables -t nat -A PREROUTING -p tcp --dport 80 -j REDIRECT --to 8080
NAT_TABLE="iptables -t nat"
REDIRECT_RULE="PREROUTING -p tcp --dport 80 -j REDIRECT --to 8080"
REDIRECT_CHECK="$NAT_TABLE -C $REDIRECT_RULE"
REDIRECT_APPEND="$NAT_TABLE -A $REDIRECT_RULE"
REDIRECT_DELETE="$NAT_TABLE -D $REDIRECT_RULE"

SAFE_EXIT_COMMAND=$(cat <<EOF
 echo "#!/bin/ash
 $REDIRECT_DELETE
 exit" > /bin/clean_rules && chmod +x /bin/clean_rules
EOF
)
ECHO_CLEAN_RULES=$(cat <<EOF
echo "
 ============================
 Before exit call clean_rules
 ============================"
EOF
)

COMMAND=$(cat <<EOF
 apk add iptables socat &&
 ($REDIRECT_CHECK || $REDIRECT_APPEND) &&
 $SAFE_EXIT_COMMAND &&
 $ECHO_CLEAN_RULES &&
 /bin/ash
EOF
)

kubectl debug $POD_NAME --profile='netadmin' -it --image=alpine -- /bin/ash -c "$COMMAND"
