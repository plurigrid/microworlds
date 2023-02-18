# We're going to hit a REST endpoint with 4 parameters that get concatenated into a single string
# the payload is base64 encoded JSON string with a single key "get_item" 

# prefix: https://lcd.uni.juno.deuslabs.fi/cosmwasm/wasm/v1/contract/
# address: juno1uedfd9nu4rc9hgfha8kswqvttd9xnrunn8nl8r3hcmjkg3r5c0jslmqsx0
# path:    /smart/
# payload: eyJnZXRfaXRlbSI6IHsia2V5IjogImNvbG9yIn19 

# use whatever library python uses to hit the end point, allowing for different parameters
# use base64 to encode the payload

import requests
import base64
import json

# export a class that can be used to query
# the contract state
class ContractState:
    def __init__(self, contract_address):
        self.contract_address = contract_address
        self.base_url = "https://lcd.uni.juno.deuslabs.fi/cosmwasm/wasm/v1/contract/"
        self.path = "/smart/"
    
    def get_item(self, key):
        payload = {
            "get_item": {
                "key": key
            }
        }
        encoded_payload = base64.b64encode(json.dumps(payload).encode()).decode()
        full_url = self.base_url + self.contract_address + self.path + encoded_payload

        try:
            response = requests.get(full_url)
            data = response.json()
            # return the nested value data.data.item
            return data["data"]["item"]
        except Exception as e:
            print(f"Error getting contract state: {e}")
            return None