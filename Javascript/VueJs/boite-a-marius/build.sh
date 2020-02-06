# Build front
cd Front
npm i
npm run build

# Build Backend
cd ../Backend
cargo build --release 
