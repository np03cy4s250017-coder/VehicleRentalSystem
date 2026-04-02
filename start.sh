#!/bin/bash
# YatraSathi Development Startup Script
# Run this from the project root directory

echo "🚗 Starting YatraSathi Development Environment..."
echo ""

# Start backend
echo "📦 Starting Rust/Axum Backend on port 3001..."
cd backend
cargo run &
BACKEND_PID=$!
cd ..

# Wait for backend to be ready
sleep 5

# Start frontend
echo "🎨 Starting SvelteKit Frontend on port 5173..."
cd frontend
npm run dev &
FRONTEND_PID=$!
cd ..

echo ""
echo "✅ YatraSathi is running!"
echo "   Frontend: http://localhost:5173"
echo "   Backend:  http://localhost:3001"
echo ""
echo "Press Ctrl+C to stop both servers"

# Wait and cleanup on exit
trap "kill $BACKEND_PID $FRONTEND_PID 2>/dev/null" EXIT
wait
