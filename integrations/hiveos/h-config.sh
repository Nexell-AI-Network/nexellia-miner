####################################################################################
###
### nexellia-miner
### https://github.com/nexellia-network/nexellia-miner/releases
###
### Hive integration: Merlin
###
####################################################################################

#!/usr/bin/env bash
[[ -e /hive/custom ]] && . /hive/custom/nexellia-miner/h-manifest.conf
[[ -e /hive/miners/custom ]] && . /hive/miners/custom/nexellia-miner/h-manifest.conf
conf=""
conf+=" --nexelliad-address=$CUSTOM_URL --mining-address $CUSTOM_TEMPLATE"


[[ ! -z $CUSTOM_USER_CONFIG ]] && conf+=" $CUSTOM_USER_CONFIG"

echo "$conf"
echo "$conf" > $CUSTOM_CONFIG_FILENAME

