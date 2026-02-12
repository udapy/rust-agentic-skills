#!/bin/bash
# A "noisy" script to test stdout piping
echo "This should NOT break the JSON-RPC stream"
>&2 echo "This stderr content should also be captured safely"
echo "Result: pollution check passed"
