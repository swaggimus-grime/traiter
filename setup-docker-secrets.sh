#!/bin/bash

echo "🔐 Setting up Docker secrets for development..."

# Create secrets directory if it doesn't exist
mkdir -p secrets

# Function to generate secret if it doesn't exist
generate_secret_if_missing() {
    local secret_file="$1"
    local description="$2"
    local generator="$3"

    if [ ! -f "secrets/$secret_file" ]; then
        echo "Generating $description..."
        eval "$generator" > "secrets/$secret_file"
        echo "✅ Generated secrets/$secret_file"
    else
        echo "⚠️  secrets/$secret_file already exists, skipping"
    fi
}

# Generate secrets
generate_secret_if_missing "db_password.txt" "database password" "openssl rand -base64 32 | tr -d '=+/' | cut -c1-25"
generate_secret_if_missing "jwt_secret.txt" "JWT secret" "openssl rand -base64 64 | tr -d '=+/' | cut -c1-50"
generate_secret_if_missing "api_key.txt" "API key" "openssl rand -hex 32"

# Create .env.public if it doesn't exist
if [ ! -f ".env.public" ]; then
    echo "Creating .env.public..."
    cat > .env.public << 'EOF'
# Public configuration for Docker development
LOG_LEVEL=debug
ENVIRONMENT=development
DATABASE_HOST=postgres
API_BASE_URL=http://localhost:8080
PORT=8080
EOF
    echo "✅ Created .env.public"
fi

# Create templates for reference
mkdir -p secrets-templates

cat > secrets-templates/db_password.txt.example << 'EOF'
your-database-password-here
EOF

cat > secrets-templates/jwt_secret.txt.example << 'EOF'
your-jwt-secret-key-here
EOF

cat > secrets-templates/api_key.txt.example << 'EOF'
your-api-key-here
EOF

echo ""
echo "🎉 Docker secrets setup complete!"
echo ""
echo "📁 Created:"
echo "   secrets/db_password.txt    (❌ not tracked)"
echo "   secrets/jwt_secret.txt     (❌ not tracked)"
echo "   secrets/api_key.txt        (❌ not tracked)"
echo "   .env.public                (✅ can be tracked)"
echo ""
echo "🚀 To start with Docker:"
echo "   docker-compose up --build"
echo ""
echo "🔍 To view secrets (for debugging):"
echo "   echo \"Database password: \$(cat secrets/db_password.txt)\""
echo "   echo \"JWT secret: \$(cat secrets/jwt_secret.txt)\""
echo ""
echo "⚠️  The secrets/ directory is in .gitignore and won't be committed."