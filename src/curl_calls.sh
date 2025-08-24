# Add some leaves
curl -X POST http://127.0.0.1:3000/add-leaf \
  -H "Content-Type: application/json" \
  -d '"0"'
curl -X POST http://127.0.0.1:3000/add-leaf \
  -H "Content-Type: application/json" \
  -d '"1"'
curl -X POST http://127.0.0.1:3000/add-leaf \
  -H "Content-Type: application/json" \
  -d '"2"'
echo ""
echo "-------------------------------"

# Get the number of leaves
curl -X GET http://127.0.0.1:3000/get-num-leaves
echo ""
echo "-------------------------------"

# Get the root hash
curl -X GET http://127.0.0.1:3000/get-root
echo ""
echo "-------------------------------"

# Get a proof for leaf at index 2
curl -X GET http://127.0.0.1:3000/get-proof \
  -H "Content-Type: application/json" \
  -d 2
echo ""
echo "-------------------------------"