#!/bin/bash

xterm -e "chr node start"

# xterm -e "node ./terminal/src/server.js"
xterm -hold -e "bun run dev"