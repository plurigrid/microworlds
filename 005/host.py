import ContractState
import Pixels

# instantiate a contract state object
# and a pixels object

c = ContractState.ContractState("juno1uedfd9nu4rc9hgfha8kswqvttd9xnrunn8nl8r3hcmjkg3r5c0jslmqsx0")
p = Pixels.Pixels("COM5")

# every 10 seconds poll the contract for a change in color

import time

while True:
    color = c.get_item("color")
    if color:
        p.set_color(color)
    time.sleep(10)
